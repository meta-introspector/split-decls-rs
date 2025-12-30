// Generated macro for DenseTransitions (struct)
macro_rules! Depcrate_nfa_thompson_nfaDenseTransitions {
() => {
// Module: crate::nfa::thompson::nfa
// Provides: {"DenseTransitions"}
// Dependencies: {}
# [doc = " A sequence of transitions used to represent a dense state."] # [doc = ""] # [doc = " This is the primary representation of a [`Dense`](State::Dense) state. It"] # [doc = " provides constant time matching. That is, given a byte in a haystack and"] # [doc = " a `DenseTransitions`, one can determine if the state matches in constant"] # [doc = " time."] # [doc = ""] # [doc = " This is in contrast to `SparseTransitions`, whose time complexity is"] # [doc = " necessarily bigger than constant time. Also in contrast, `DenseTransitions`"] # [doc = " usually requires (much) more heap memory."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct DenseTransitions { # [doc = " A dense representation of this state's transitions on the heap. This"] # [doc = " always has length 256."] pub transitions : Box < [StateID] > , }
};
}
