// Generated macro for impl_206 (impl)
macro_rules! Depcrate_ttyimpl_206 {
() => {
// Module: crate::tty
// Provides: {"impl_206"}
// Dependencies: {}
# [doc = " On UNIX, the `isatty()` function returns true if a file"] # [doc = " descriptor is a terminal."] # [cfg (all (unix , feature = "libc"))] impl < S : AsRawFd > IsTty for S { fn is_tty (& self) -> bool { let fd = self . as_raw_fd () ; unsafe { libc :: isatty (fd) == 1 } } }
};
}
