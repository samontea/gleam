use crate::ast::SrcSpan;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LexicalError {
    pub error: LexicalErrorType,
    pub location: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LexicalErrorType {
    BadStringEscape,       // string contains an unescaped slash
    DigitOutOfRadix,       // 0x012 , 2 is out of radix
    NumTrailingUnderscore, // 1_000_ is not allowed
    RadixIntNoValue,       // 0x, 0b, 0o without a value
    UnexpectedStringEnd,   // Unterminated string literal
    UnrecognizedToken { tok: char },
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub error: ParseErrorType,
    pub location: SrcSpan,
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
    ExpectedExpr,            // after "->" in a case clause
    ExpectedName,            // any token used when a Name was expected
    ExpectedPattern,         // after ':' where a pattern is expected
    ExpectedType,            // after ':' or '->' where a type annotation is expected
    ExpectedUpName,          // any token used when a UpName was expected
    ExpectedValue,           // no value after "="
    ExprLparStart,           // it seems "(" was used to start an expression
    ExprTailBinding,         // a binding in the tail position of an expression sequence
    ExtraSeparator,          // tuple(1,,) <- the 2nd comma is an extra separator
    IncorrectName,           // UpName or DiscardName used when Name was expected
    IncorrectUpName,         // Name or DiscardName used when UpName was expected
    InvalidBitStringSegment, // <<7:hello>> `hello` is an invalid bitstring segment
    InvalidTailPattern,      // only name and _name are allowed after ".." in list pattern
    InvalidTupleAccess,      // only positive int literals for tuple access
    LexError { error: LexicalError },
    ListNilNotAllowed, // [] is not allowed here
    NoConstructors,    // A type "A {}" must have at least one constructor
    NoCaseClause,      // a case with no claueses
    NoExpression,      // between "{" and "}" in expression position, there must be an expression
    NoValueAfterEqual, // = <something other than a value>
    NotConstType,      // :fn(), name, _  are not valid const types
    OpNakedRight,      // Operator with no value to the right
    OpaqueTypeAlias,   // Type aliases cannot be opaque
    TooManyArgHoles,   // a function call can have at most 1 arg hole
    UnexpectedEOF,
    UnexpectedReservedWord, // reserved word used when a name was expected
    UnexpectedToken { expected: Vec<String> },
}

impl LexicalError {
    pub fn to_parse_error_info(&self) -> (&str, Vec<String>) {
        match self.error {
            LexicalErrorType::BadStringEscape => (
                "I don't understand this escape code",
                vec![
                    "Hint: Add another backslash before it.".to_string(),
                    "See: https://gleam.run/book/tour/strings.html#escape-sequences".to_string(),
                ],
            ),
            LexicalErrorType::DigitOutOfRadix => {
                ("This digit is too big for the specified radix.", vec![])
            }
            LexicalErrorType::NumTrailingUnderscore => (
                "Numbers cannot have a trailing underscore.",
                vec!["Hint: remove it.".to_string()],
            ),
            LexicalErrorType::RadixIntNoValue => ("This integer has no value.", vec![]),
            LexicalErrorType::UnexpectedStringEnd => {
                ("The string starting here was left open.", vec![])
            }
            LexicalErrorType::UnrecognizedToken { .. } => (
                "I can't figure out what to do with this character.",
                vec!["Hint: Is it a typo?".to_string()],
            ),
        }
    }
}
