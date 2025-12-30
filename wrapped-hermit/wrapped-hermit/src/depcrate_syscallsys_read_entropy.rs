// Generated macro for sys_read_entropy (function)
macro_rules! Depcrate_syscallsys_read_entropy {
() => {
// Module: crate::syscall
// Provides: {"sys_read_entropy"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_read_entropy (buf : * mut u8 , len : usize , flags : u32) -> isize { let result : isize = syscall ! (SyscallNo :: ReadEntropy , buf , len , flags) . try_into () . unwrap () ; if result < 0 { unsafe { ERRNO . get () . write ((- result) . try_into () . unwrap ()) ; } } else { unsafe { ERRNO . get () . write (0) ; } } result }
};
}
