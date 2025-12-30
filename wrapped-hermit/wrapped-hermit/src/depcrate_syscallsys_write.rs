// Generated macro for sys_write (function)
macro_rules! Depcrate_syscallsys_write {
() => {
// Module: crate::syscall
// Provides: {"sys_write"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_write (fd : i32 , buf : * const u8 , len : usize) -> isize { let result : isize = syscall ! (SyscallNo :: Write , fd , buf , len) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write ((- result) . try_into () . unwrap ()) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
