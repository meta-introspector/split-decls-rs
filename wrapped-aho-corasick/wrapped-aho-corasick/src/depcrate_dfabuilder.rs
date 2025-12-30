// Generated macro for Builder (struct)
macro_rules! Depcrate_dfaBuilder {
() => {
// Module: crate::dfa
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder for configuring an Aho-Corasick DFA."] # [doc = ""] # [doc = " This builder has a subset of the options available to a"] # [doc = " [`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,"] # [doc = " their behavior is identical."] # [derive (Clone , Debug)] pub struct Builder { noncontiguous : noncontiguous :: Builder , start_kind : StartKind , byte_classes : bool , }
};
}
