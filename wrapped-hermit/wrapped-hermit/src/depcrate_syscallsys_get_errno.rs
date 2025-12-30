// Generated macro for sys_get_errno (function)
macro_rules! Depcrate_syscallsys_get_errno {
() => {
// Module: crate::syscall
// Provides: {"sys_get_errno"}
// Dependencies: {}
# [doc = " Get the last error number from the thread local storage"] # [no_mangle] pub extern "C" fn sys_get_errno () -> i32 { unsafe { ERRNO . get () . read () } }
};
}
