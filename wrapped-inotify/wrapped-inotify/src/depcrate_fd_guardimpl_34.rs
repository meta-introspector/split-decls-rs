// Generated macro for impl_34 (impl)
macro_rules! Depcrate_fd_guardimpl_34 {
() => {
// Module: crate::fd_guard
// Provides: {"impl_34"}
// Dependencies: {}
impl Drop for FdGuard { fn drop (& mut self) { if self . close_on_drop . load (Ordering :: Acquire) { unsafe { ffi :: close (self . fd) ; } } } }
};
}
