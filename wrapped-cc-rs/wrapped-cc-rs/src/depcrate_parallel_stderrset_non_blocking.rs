// Generated macro for set_non_blocking (function)
macro_rules! Depcrate_parallel_stderrset_non_blocking {
() => {
// Module: crate::parallel::stderr
// Provides: {"set_non_blocking"}
// Dependencies: {}
# [cfg (unix)] pub fn set_non_blocking (pipe : & impl std :: os :: unix :: io :: AsRawFd) -> Result < () , Error > { let fd = pipe . as_raw_fd () ; let flags = get_flags (fd) ? ; set_flags (fd , flags | libc :: O_NONBLOCK) }
};
}
