// Generated macro for syscall_futex (function)
macro_rules! Depcrate_tests_futexsyscall_futex {
() => {
// Module: crate::tests::futex
// Provides: {"syscall_futex"}
// Dependencies: {}
fn syscall_futex (futex : * const u32 , op : libc :: c_int , val : u32) -> io :: Result < i64 > { let ret = unsafe { libc :: syscall (libc :: SYS_futex , futex , op , val , ptr :: null :: < u8 > () , ptr :: null :: < u8 > () , 0u32 ,) } ; if ret >= 0 { Ok (ret as _) } else { Err (io :: Error :: from_raw_os_error (- ret as _)) } }
};
}
