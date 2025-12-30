// Generated macro for impl_36 (impl)
macro_rules! Depcrate_fd_guardimpl_36 {
() => {
// Module: crate::fd_guard
// Provides: {"impl_36"}
// Dependencies: {}
impl IntoRawFd for FdGuard { fn into_raw_fd (self) -> RawFd { self . should_not_close () ; self . fd } }
};
}
