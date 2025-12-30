// Generated macro for ParseError (struct)
macro_rules! Depcrate_unicodeset_parse_parseParseError {
() => {
// Module: crate::unicodeset_parse::parse
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " The error type returned by the `parse` functions in this crate."] # [doc = ""] # [doc = " See [`ParseError::fmt_with_source`] for pretty-printing and [`ParseErrorKind`] of the"] # [doc = " different types of errors represented by this struct."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct ParseError { offset : Option < usize > , kind : ParseErrorKind , }
};
}
