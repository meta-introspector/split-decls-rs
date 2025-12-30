// Generated macro for impl_27 (impl)
macro_rules! Depcrate_state_definitionsimpl_27 {
() => {
// Module: crate::state::definitions
// Provides: {"impl_27"}
// Dependencies: {}
impl TryFrom < u8 > for State { type Error = u8 ; # [inline (always)] fn try_from (raw : u8) -> Result < Self , Self :: Error > { STATES . get (raw as usize) . ok_or (raw) . copied () } }
};
}
