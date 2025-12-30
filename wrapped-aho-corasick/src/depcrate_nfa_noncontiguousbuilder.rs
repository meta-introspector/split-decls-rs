// Generated macro for Builder (struct)
macro_rules! Depcrate_nfa_noncontiguousBuilder {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A builder for configuring an Aho-Corasick noncontiguous NFA."] # [doc = ""] # [doc = " This builder has a subset of the options available to a"] # [doc = " [`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,"] # [doc = " their behavior is identical."] # [derive (Clone , Debug)] pub struct Builder { match_kind : MatchKind , prefilter : bool , ascii_case_insensitive : bool , dense_depth : usize , }
};
}
