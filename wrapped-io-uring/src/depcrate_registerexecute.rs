// Generated macro for execute (function)
macro_rules! Depcrate_registerexecute {
() => {
// Module: crate::register
// Provides: {"execute"}
// Dependencies: {}
pub (crate) fn execute (fd : RawFd , opcode : libc :: c_uint , arg : * const libc :: c_void , len : libc :: c_uint ,) -> io :: Result < i32 > { unsafe { sys :: io_uring_register (fd , opcode , arg , len) } }
};
}
