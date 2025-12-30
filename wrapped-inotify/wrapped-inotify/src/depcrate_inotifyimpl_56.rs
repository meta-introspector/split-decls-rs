// Generated macro for impl_56 (impl)
macro_rules! Depcrate_inotifyimpl_56 {
() => {
// Module: crate::inotify
// Provides: {"impl_56"}
// Dependencies: {}
impl From < OwnedFd > for Inotify { fn from (fd : OwnedFd) -> Inotify { unsafe { Inotify :: from_raw_fd (fd . into_raw_fd ()) } } }
};
}
