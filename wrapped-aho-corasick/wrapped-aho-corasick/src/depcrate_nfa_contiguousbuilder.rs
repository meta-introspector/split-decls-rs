// Generated macro for Builder (struct)
macro_rules! Depcrate_nfa_contiguousBuilder {
() => {
// Module: crate::nfa::contiguous
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder for configuring an Aho-Corasick contiguous NFA."] # [doc = ""] # [doc = " This builder has a subset of the options available to a"] # [doc = " [`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,"] # [doc = " their behavior is identical."] # [derive (Clone , Debug)] pub struct Builder { noncontiguous : noncontiguous :: Builder , dense_depth : usize , byte_classes : bool , }
};
}
