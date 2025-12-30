// Generated macro for BuildError (struct)
macro_rules! Depcrate_dfa_onepassBuildError {
() => {
// Module: crate::dfa::onepass
// Provides: {"BuildError"}
// Dependencies: {}
# [doc = " An error that occurred during the construction of a one-pass DFA."] # [doc = ""] # [doc = " This error does not provide many introspection capabilities. There are"] # [doc = " generally only two things you can do with it:"] # [doc = ""] # [doc = " * Obtain a human readable message via its `std::fmt::Display` impl."] # [doc = " * Access an underlying [`thompson::BuildError`] type from its `source`"] # [doc = " method via the `std::error::Error` trait. This error only occurs when using"] # [doc = " convenience routines for building a one-pass DFA directly from a pattern"] # [doc = " string."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements the `std::error::Error`"] # [doc = " trait."] # [derive (Clone , Debug)] pub struct BuildError { kind : BuildErrorKind , }
};
}
