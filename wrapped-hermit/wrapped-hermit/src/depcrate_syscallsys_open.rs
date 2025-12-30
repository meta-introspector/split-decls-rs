// Generated macro for sys_open (function)
macro_rules! Depcrate_syscallsys_open {
() => {
// Module: crate::syscall
// Provides: {"sys_open"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_open (name : * const i8 , flags : i32 , mode : i32) -> i32 { let result : i32 = syscall ! (SyscallNo :: Open , name , flags , mode) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write (- result) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
