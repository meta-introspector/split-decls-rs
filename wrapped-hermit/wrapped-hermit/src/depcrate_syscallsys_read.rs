// Generated macro for sys_read (function)
macro_rules! Depcrate_syscallsys_read {
() => {
// Module: crate::syscall
// Provides: {"sys_read"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_read (fd : i32 , buf : * mut u8 , len : usize) -> isize { let result : isize = syscall ! (SyscallNo :: Read , fd , buf , len) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write ((- result) . try_into () . unwrap ()) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
