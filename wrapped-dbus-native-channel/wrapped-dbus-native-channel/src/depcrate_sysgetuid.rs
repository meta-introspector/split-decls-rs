// Generated macro for getuid (function)
macro_rules! Depcrate_sysgetuid {
() => {
// Module: crate::sys
// Provides: {"getuid"}
// Dependencies: {}
pub fn getuid () -> u32 { let x = unsafe { libc :: getuid () } ; x as u32 }
};
}
