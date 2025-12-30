// Generated macro for State (struct)
macro_rules! Depcrate_hir_literalState {
() => {
// Module: crate::hir::literal
// Provides: {"State"}
// Dependencies: {}
# [doc = " A single state in a trie. Uses a sparse representation for its transitions."] # [derive (Debug , Default)] struct State { # [doc = " Sparse representation of the transitions out of this state. Transitions"] # [doc = " are sorted by byte. There is at most one such transition for any"] # [doc = " particular byte."] trans : Vec < (u8 , usize) > , }
};
}
