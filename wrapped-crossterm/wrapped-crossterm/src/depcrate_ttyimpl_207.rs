// Generated macro for impl_207 (impl)
macro_rules! Depcrate_ttyimpl_207 {
() => {
// Module: crate::tty
// Provides: {"impl_207"}
// Dependencies: {}
# [cfg (all (unix , not (feature = "libc")))] impl < S : AsRawFd > IsTty for S { fn is_tty (& self) -> bool { let fd = self . as_raw_fd () ; rustix :: termios :: isatty (unsafe { std :: os :: unix :: io :: BorrowedFd :: borrow_raw (fd) }) } }
};
}
