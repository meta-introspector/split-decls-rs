// Generated macro for impl_55 (impl)
macro_rules! Depcrate_inotifyimpl_55 {
() => {
// Module: crate::inotify
// Provides: {"impl_55"}
// Dependencies: {}
impl From < Inotify > for OwnedFd { fn from (fd : Inotify) -> OwnedFd { unsafe { OwnedFd :: from_raw_fd (fd . into_raw_fd ()) } } }
};
}
