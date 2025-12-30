// Generated macro for impl_30 (impl)
macro_rules! Depcrate_state_definitionsimpl_30 {
() => {
// Module: crate::state::definitions
// Provides: {"impl_30"}
// Dependencies: {}
impl TryFrom < u8 > for Action { type Error = u8 ; # [inline (always)] fn try_from (raw : u8) -> Result < Self , Self :: Error > { ACTIONS . get (raw as usize) . ok_or (raw) . copied () } }
};
}
