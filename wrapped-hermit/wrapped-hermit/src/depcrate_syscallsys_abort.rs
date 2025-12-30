// Generated macro for sys_abort (function)
macro_rules! Depcrate_syscallsys_abort {
() => {
// Module: crate::syscall
// Provides: {"sys_abort"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_abort () -> ! { sys_exit (1) }
};
}
