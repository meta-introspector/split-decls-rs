// Generated macro for state_change (function)
macro_rules! Depcrate_statestate_change {
() => {
// Module: crate::state
// Provides: {"state_change"}
// Dependencies: {}
# [doc = " Transition to next [`State`]"] # [doc = ""] # [doc = " Note: This does not directly support UTF-8."] # [doc = " - If the data is validated as UTF-8 (e.g. `str`) or single-byte C1 control codes are"] # [doc = "   unsupported, then treat [`Action::BeginUtf8`] and [`Action::Execute`] for UTF-8 continuations"] # [doc = "   as [`Action::Print`]."] # [doc = " - If the data is not validated, then a UTF-8 state machine will need to be implemented on top,"] # [doc = "   starting with [`Action::BeginUtf8`]."] # [doc = ""] # [doc = " Note: When [`State::Anywhere`] is returned, revert back to the prior state."] # [inline] pub const fn state_change (state : State , byte : u8) -> (State , Action) { let mut change = state_change_ (State :: Anywhere , byte) ; if change == 0 { change = state_change_ (state , byte) ; } unpack (change) }
};
}
