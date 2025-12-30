// Generated macro for State (struct)
macro_rules! Depcrate_nfa_thompson_range_trieState {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"State"}
// Dependencies: {}
# [doc = " A single state in this trie."] # [derive (Clone)] struct State { # [doc = " A sorted sequence of non-overlapping transitions to other states. Each"] # [doc = " transition corresponds to a single range of bytes."] transitions : Vec < Transition > , }
};
}
