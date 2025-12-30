// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that occurred during parsing or compiling a regular expression."] # [doc = ""] # [doc = " A parse error occurs when the syntax of the regex pattern is not"] # [doc = " valid. Otherwise, a regex can still fail to build if it would"] # [doc = " result in a machine that exceeds the configured size limit, via"] # [doc = " [`RegexBuilder::size_limit`](crate::RegexBuilder::size_limit)."] # [doc = ""] # [doc = " This error type provides no introspection capabilities. The only thing you"] # [doc = " can do with it is convert it to a string as a human readable error message."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Error { msg : & 'static str , }
};
}
