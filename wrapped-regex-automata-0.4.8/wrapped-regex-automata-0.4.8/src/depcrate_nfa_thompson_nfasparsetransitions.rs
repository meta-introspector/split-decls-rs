// Generated macro for SparseTransitions (struct)
macro_rules! Depcrate_nfa_thompson_nfaSparseTransitions {
() => {
// Module: crate::nfa::thompson::nfa
// Provides: {"SparseTransitions"}
// Dependencies: {}
# [doc = " A sequence of transitions used to represent a sparse state."] # [doc = ""] # [doc = " This is the primary representation of a [`Sparse`](State::Sparse) state."] # [doc = " It corresponds to a sorted sequence of transitions with non-overlapping"] # [doc = " byte ranges. If the byte at the current position in the haystack matches"] # [doc = " one of the byte ranges, then the finite state machine should take the"] # [doc = " corresponding transition."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct SparseTransitions { # [doc = " The sorted sequence of non-overlapping transitions."] pub transitions : Box < [Transition] > , }
};
}
