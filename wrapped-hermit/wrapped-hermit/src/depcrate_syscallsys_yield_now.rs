// Generated macro for sys_yield_now (function)
macro_rules! Depcrate_syscallsys_yield_now {
() => {
// Module: crate::syscall
// Provides: {"sys_yield_now"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_yield_now () { syscall ! (SyscallNo :: Yield) ; }
};
}
