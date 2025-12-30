// Generated macro for state_change_ (function)
macro_rules! Depcrate_statestate_change_ {
() => {
// Module: crate::state
// Provides: {"state_change_"}
// Dependencies: {}
# [inline] const fn state_change_ (state : State , byte : u8) -> u8 { let state_idx = state as usize ; let byte_idx = byte as usize ; table :: STATE_CHANGES [state_idx] [byte_idx] }
};
}
