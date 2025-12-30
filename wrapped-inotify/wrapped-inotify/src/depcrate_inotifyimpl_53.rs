// Generated macro for impl_53 (impl)
macro_rules! Depcrate_inotifyimpl_53 {
() => {
// Module: crate::inotify
// Provides: {"impl_53"}
// Dependencies: {}
impl IntoRawFd for Inotify { # [inline] fn into_raw_fd (self) -> RawFd { self . fd . should_not_close () ; self . fd . fd } }
};
}
