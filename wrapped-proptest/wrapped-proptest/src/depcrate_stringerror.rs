// Generated macro for Error (enum)
macro_rules! Depcrate_stringError {
() => {
// Module: crate::string
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors which may occur when preparing a regular expression for use with"] # [doc = " string generation."] # [derive (Debug)] pub enum Error { # [doc = " The string passed as the regex was not syntactically valid."] RegexSyntax (ParseError) , # [doc = " The regex was syntactically valid, but contains elements not"] # [doc = " supported by proptest."] UnsupportedRegex (& 'static str) , }
};
}
