// Generated macro for io_uring_setup (function)
macro_rules! Depcrate_sysio_uring_setup {
() => {
// Module: crate::sys
// Provides: {"io_uring_setup"}
// Dependencies: {}
# [cfg (feature = "direct-syscall")] pub unsafe fn io_uring_setup (entries : c_uint , p : * mut io_uring_params) -> io :: Result < c_int > { to_result (sc :: syscall2 (SYSCALL_SETUP as usize , entries as usize , p as usize) as _) }
};
}
