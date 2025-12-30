// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " The kind of an error that can occur."] # [derive (Clone , Debug)] # [non_exhaustive] pub enum ErrorKind { # [doc = " An error that occurred as a result of parsing a regular expression."] # [doc = " This can be a syntax error or an error that results from attempting to"] # [doc = " compile a regular expression that is too big."] # [doc = ""] # [doc = " The string here is the underlying error converted to a string."] Regex (String) , # [doc = " An error that occurs when a building a regex that isn't permitted to"] # [doc = " match a line terminator. In general, building the regex will do its"] # [doc = " best to make matching a line terminator impossible (e.g., by removing"] # [doc = " `\\n` from the `\\s` character class), but if the regex contains a"] # [doc = " `\\n` literal, then there is no reasonable choice that can be made and"] # [doc = " therefore an error is reported."] # [doc = ""] # [doc = " The string is the literal sequence found in the regex that is not"] # [doc = " allowed."] NotAllowed (String) , # [doc = " This error occurs when a non-ASCII line terminator was provided."] # [doc = ""] # [doc = " The invalid byte is included in this error."] InvalidLineTerminator (u8) , # [doc = " Occurs when a banned byte was found in a pattern."] Banned (u8) , }
};
}
