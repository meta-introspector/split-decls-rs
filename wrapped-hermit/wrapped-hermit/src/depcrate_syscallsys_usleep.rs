// Generated macro for sys_usleep (function)
macro_rules! Depcrate_syscallsys_usleep {
() => {
// Module: crate::syscall
// Provides: {"sys_usleep"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_usleep (usecs : u64) { syscall ! (SyscallNo :: Usleep , usecs) ; }
};
}
