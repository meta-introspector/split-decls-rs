// Generated macro for BuildError (struct)
macro_rules! Depcrate_dfa_denseBuildError {
() => {
// Module: crate::dfa::dense
// Provides: {"BuildError"}
// Dependencies: {}
# [doc = " An error that occurred during the construction of a DFA."] # [doc = ""] # [doc = " This error does not provide many introspection capabilities. There are"] # [doc = " generally only two things you can do with it:"] # [doc = ""] # [doc = " * Obtain a human readable message via its `std::fmt::Display` impl."] # [doc = " * Access an underlying [`nfa::thompson::BuildError`](thompson::BuildError)"] # [doc = " type from its `source` method via the `std::error::Error` trait. This error"] # [doc = " only occurs when using convenience routines for building a DFA directly"] # [doc = " from a pattern string."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements the `std::error::Error`"] # [doc = " trait."] # [cfg (feature = "dfa-build")] # [derive (Clone , Debug)] pub struct BuildError { kind : BuildErrorKind , }
};
}
