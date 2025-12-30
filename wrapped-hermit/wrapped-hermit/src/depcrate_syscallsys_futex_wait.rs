// Generated macro for sys_futex_wait (function)
macro_rules! Depcrate_syscallsys_futex_wait {
() => {
// Module: crate::syscall
// Provides: {"sys_futex_wait"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_futex_wait (address : * mut u32 , expected : u32 , timeout : * const abi :: timespec , flags : u32 ,) -> i32 { let result : i32 = syscall ! (SyscallNo :: FutexWait , address , expected , timeout , flags) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write (- result) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
