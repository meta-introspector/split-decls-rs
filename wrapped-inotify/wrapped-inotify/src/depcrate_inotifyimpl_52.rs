// Generated macro for impl_52 (impl)
macro_rules! Depcrate_inotifyimpl_52 {
() => {
// Module: crate::inotify
// Provides: {"impl_52"}
// Dependencies: {}
impl FromRawFd for Inotify { unsafe fn from_raw_fd (fd : RawFd) -> Self { Inotify { fd : Arc :: new (FdGuard :: from_raw_fd (fd)) , } } }
};
}
