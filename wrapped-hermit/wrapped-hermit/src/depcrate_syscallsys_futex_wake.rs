// Generated macro for sys_futex_wake (function)
macro_rules! Depcrate_syscallsys_futex_wake {
() => {
// Module: crate::syscall
// Provides: {"sys_futex_wake"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_futex_wake (address : * mut u32 , count : i32) -> i32 { let result : i32 = syscall ! (SyscallNo :: FutexWake , address , count) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write (- result) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
