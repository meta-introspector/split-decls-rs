// Generated macro for STATE_QUIT (const)
macro_rules! Depcrate_dfaSTATE_QUIT {
() => {
// Module: crate::dfa
// Provides: {"STATE_QUIT"}
// Dependencies: {}
# [doc = " A quit state means that the DFA came across some input that it doesn't"] # [doc = " know how to process correctly. The DFA should quit and another matching"] # [doc = " engine should be run in its place."] const STATE_QUIT : StatePtr = STATE_DEAD + 1 ;
};
}
