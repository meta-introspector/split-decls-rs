// Generated macro for BuildError (struct)
macro_rules! Depcrate_util_errorBuildError {
() => {
// Module: crate::util::error
// Provides: {"BuildError"}
// Dependencies: {}
# [doc = " An error that occurred during the construction of an Aho-Corasick"] # [doc = " automaton."] # [doc = ""] # [doc = " Build errors occur when some kind of limit has been exceeded, either in the"] # [doc = " number of states, the number of patterns of the length of a pattern. These"] # [doc = " limits aren't part of the public API, but they should generally be large"] # [doc = " enough to handle most use cases."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements the `std::error::Error`"] # [doc = " trait."] # [derive (Clone , Debug)] pub struct BuildError { kind : ErrorKind , }
};
}
