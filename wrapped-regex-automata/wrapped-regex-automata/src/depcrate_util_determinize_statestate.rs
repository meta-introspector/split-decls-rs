// Generated macro for State (struct)
macro_rules! Depcrate_util_determinize_stateState {
() => {
// Module: crate::util::determinize::state
// Provides: {"State"}
// Dependencies: {}
# [doc = " A DFA state that, at its core, is represented by an ordered set of NFA"] # [doc = " states."] # [doc = ""] # [doc = " This type is intended to be used only in NFA-to-DFA conversion via powerset"] # [doc = " construction."] # [doc = ""] # [doc = " It may be cheaply cloned and accessed safely from multiple threads"] # [doc = " simultaneously."] # [derive (Clone , Eq , Hash , PartialEq , PartialOrd , Ord)] pub (crate) struct State (Arc < [u8] >) ;
};
}
