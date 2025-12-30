// Generated macro for BuildError (struct)
macro_rules! Depcrate_meta_errorBuildError {
() => {
// Module: crate::meta::error
// Provides: {"BuildError"}
// Dependencies: {}
# [doc = " An error that occurs when construction of a `Regex` fails."] # [doc = ""] # [doc = " A build error is generally a result of one of two possible failure"] # [doc = " modes. First is a parse or syntax error in the concrete syntax of a"] # [doc = " pattern. Second is that the construction of the underlying regex matcher"] # [doc = " fails, usually because it gets too big with respect to limits like"] # [doc = " [`Config::nfa_size_limit`](crate::meta::Config::nfa_size_limit)."] # [doc = ""] # [doc = " This error provides very little introspection capabilities. You can:"] # [doc = ""] # [doc = " * Ask for the [`PatternID`] of the pattern that caused an error, if one"] # [doc = " is available. This is available for things like syntax errors, but not for"] # [doc = " cases where build limits are exceeded."] # [doc = " * Ask for the underlying syntax error, but only if the error is a syntax"] # [doc = " error."] # [doc = " * Ask for a human readable message corresponding to the underlying error."] # [doc = " * The `BuildError::source` method (from the `std::error::Error`"] # [doc = " trait implementation) may be used to query for an underlying error if one"] # [doc = " exists. There are no API guarantees about which error is returned."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements `std::error::Error`."] # [derive (Clone , Debug)] pub struct BuildError { kind : BuildErrorKind , }
};
}
