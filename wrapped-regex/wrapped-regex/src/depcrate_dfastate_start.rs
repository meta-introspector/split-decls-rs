// Generated macro for STATE_START (const)
macro_rules! Depcrate_dfaSTATE_START {
() => {
// Module: crate::dfa
// Provides: {"STATE_START"}
// Dependencies: {}
# [doc = " A start state is a state that the DFA can start in."] # [doc = ""] # [doc = " Note that start states have their lower bits set to a state pointer."] const STATE_START : StatePtr = 1 << 30 ;
};
}
