// Generated macro for sys_exit (function)
macro_rules! Depcrate_syscallsys_exit {
() => {
// Module: crate::syscall
// Provides: {"sys_exit"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_exit (arg : i32) -> ! { syscall ! (SyscallNo :: Exit , arg) ; unreachable ! () }
};
}
