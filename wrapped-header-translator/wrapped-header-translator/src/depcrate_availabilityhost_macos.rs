// Generated macro for HOST_MACOS (const)
macro_rules! Depcrate_availabilityHOST_MACOS {
() => {
// Module: crate::availability
// Provides: {"HOST_MACOS"}
// Dependencies: {}
pub const HOST_MACOS : u32 = if option_env ! ("CI") . is_some () { 9999 } else { 14 } ;
};
}
