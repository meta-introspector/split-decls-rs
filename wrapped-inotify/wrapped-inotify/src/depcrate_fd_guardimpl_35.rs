// Generated macro for impl_35 (impl)
macro_rules! Depcrate_fd_guardimpl_35 {
() => {
// Module: crate::fd_guard
// Provides: {"impl_35"}
// Dependencies: {}
impl FromRawFd for FdGuard { unsafe fn from_raw_fd (fd : RawFd) -> Self { FdGuard { fd , close_on_drop : AtomicBool :: new (true) , } } }
};
}
