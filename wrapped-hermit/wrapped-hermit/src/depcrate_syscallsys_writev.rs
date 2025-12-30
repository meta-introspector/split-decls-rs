// Generated macro for sys_writev (function)
macro_rules! Depcrate_syscallsys_writev {
() => {
// Module: crate::syscall
// Provides: {"sys_writev"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_writev (fd : i32 , iov : * const u8 , iovcnt : usize) -> isize { let result : isize = syscall ! (SyscallNo :: Writev , fd , iov , iovcnt) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write ((- result) . try_into () . unwrap ()) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
