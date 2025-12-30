// Generated macro for Compiler (struct)
macro_rules! Depcrate_nfa_noncontiguousCompiler {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"Compiler"}
// Dependencies: {}
# [doc = " A compiler uses a builder configuration and builds up the NFA formulation"] # [doc = " of an Aho-Corasick automaton. This roughly corresponds to the standard"] # [doc = " formulation described in textbooks, with some tweaks to support leftmost"] # [doc = " searching."] # [derive (Debug)] struct Compiler < 'a > { builder : & 'a Builder , prefilter : prefilter :: Builder , nfa : NFA , byteset : ByteClassSet , }
};
}
