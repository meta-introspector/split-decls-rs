// Generated macro for STATE_DEAD (const)
macro_rules! Depcrate_dfaSTATE_DEAD {
() => {
// Module: crate::dfa
// Provides: {"STATE_DEAD"}
// Dependencies: {}
# [doc = " A dead state means that the state has been computed and it is known that"] # [doc = " once it is entered, no future match can ever occur."] const STATE_DEAD : StatePtr = STATE_UNKNOWN + 1 ;
};
}
