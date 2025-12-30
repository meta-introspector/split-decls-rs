// Generated macro for Transition (struct)
macro_rules! Depcrate_nfa_noncontiguousTransition {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"Transition"}
// Dependencies: {}
# [doc = " A single transition in a non-contiguous NFA."] # [derive (Clone , Copy , Default)] # [repr (packed)] pub (crate) struct Transition { byte : u8 , next : StateID , link : StateID , }
};
}
