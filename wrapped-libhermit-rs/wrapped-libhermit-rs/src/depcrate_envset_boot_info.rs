// Generated macro for set_boot_info (function)
macro_rules! Depcrate_envset_boot_info {
() => {
// Module: crate::env
// Provides: {"set_boot_info"}
// Dependencies: {}
pub fn set_boot_info (raw_boot_info : RawBootInfo) { let boot_info = BootInfo :: from (raw_boot_info) ; BOOT_INFO . set (boot_info) . unwrap () ; }
};
}
