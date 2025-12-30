// Generated macro for umask (function)
macro_rules! Depcrate_fsumask {
() => {
// Module: crate::fs
// Provides: {"umask"}
// Dependencies: {}
pub fn umask (new_mask : AccessPermission) -> AccessPermission { let mut lock = UMASK . lock () ; let old = * lock ; * lock = new_mask ; old }
};
}
