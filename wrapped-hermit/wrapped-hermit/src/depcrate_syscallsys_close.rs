// Generated macro for sys_close (function)
macro_rules! Depcrate_syscallsys_close {
() => {
// Module: crate::syscall
// Provides: {"sys_close"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_close (fd : i32) -> i32 { let result : i32 = syscall ! (SyscallNo :: Close , fd) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write (- result) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
