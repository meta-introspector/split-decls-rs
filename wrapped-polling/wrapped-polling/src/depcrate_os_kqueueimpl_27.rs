// Generated macro for impl_27 (impl)
macro_rules! Depcrate_os_kqueueimpl_27 {
() => {
// Module: crate::os::kqueue
// Provides: {"impl_27"}
// Dependencies: {}
impl Drop for Poller { fn drop (& mut self) { # [cfg (feature = "tracing")] let span = tracing :: trace_span ! ("drop" , kqueue_fd = ? self . kqueue_fd . as_raw_fd () ,) ; # [cfg (feature = "tracing")] let _enter = span . enter () ; let _ = self . notify . deregister (self) ; } }
};
}
