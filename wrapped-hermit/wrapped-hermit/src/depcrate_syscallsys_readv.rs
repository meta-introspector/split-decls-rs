// Generated macro for sys_readv (function)
macro_rules! Depcrate_syscallsys_readv {
() => {
// Module: crate::syscall
// Provides: {"sys_readv"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_readv (fd : i32 , iov : * const u8 , iovcnt : usize) -> isize { let result : isize = syscall ! (SyscallNo :: Readv , fd , iov , iovcnt) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write ((- result) . try_into () . unwrap ()) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
