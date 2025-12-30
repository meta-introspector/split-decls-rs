// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " The kind of an error that can occur."] # [derive (Clone , Debug)] # [non_exhaustive] pub enum ErrorKind { # [doc = " An error that occurred as a result of parsing a regular expression."] # [doc = " This can be a syntax error or an error that results from attempting to"] # [doc = " compile a regular expression that is too big."] # [doc = ""] # [doc = " The string here is the underlying error converted to a string."] Regex (String) , }
};
}
