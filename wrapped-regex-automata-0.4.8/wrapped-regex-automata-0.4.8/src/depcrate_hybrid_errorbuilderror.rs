// Generated macro for BuildError (struct)
macro_rules! Depcrate_hybrid_errorBuildError {
() => {
// Module: crate::hybrid::error
// Provides: {"BuildError"}
// Dependencies: {}
# [doc = " An error that occurs when initial construction of a lazy DFA fails."] # [doc = ""] # [doc = " A build error can occur when insufficient cache capacity is configured or"] # [doc = " if something about the NFA is unsupported. (For example, if one attempts"] # [doc = " to build a lazy DFA without heuristic Unicode support but with an NFA that"] # [doc = " contains a Unicode word boundary.)"] # [doc = ""] # [doc = " This error does not provide many introspection capabilities. There are"] # [doc = " generally only two things you can do with it:"] # [doc = ""] # [doc = " * Obtain a human readable message via its `std::fmt::Display` impl."] # [doc = " * Access an underlying"] # [doc = " [`nfa::thompson::BuildError`](crate::nfa::thompson::BuildError)"] # [doc = " type from its `source` method via the `std::error::Error` trait. This error"] # [doc = " only occurs when using convenience routines for building a lazy DFA"] # [doc = " directly from a pattern string."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements the `std::error::Error`"] # [doc = " trait."] # [derive (Clone , Debug)] pub struct BuildError { kind : BuildErrorKind , }
};
}
