// Generated macro for checked_round_up (function)
macro_rules! Depcrate_machinst_abichecked_round_up {
() => {
// Module: crate::machinst::abi
// Provides: {"checked_round_up"}
// Dependencies: {}
fn checked_round_up (val : u32 , mask : u32) -> Option < u32 > { Some (val . checked_add (mask) ? & ! mask) }
};
}
