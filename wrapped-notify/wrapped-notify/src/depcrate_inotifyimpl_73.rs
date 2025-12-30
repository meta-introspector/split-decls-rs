// Generated macro for impl_73 (impl)
macro_rules! Depcrate_inotifyimpl_73 {
() => {
// Module: crate::inotify
// Provides: {"impl_73"}
// Dependencies: {}
impl Drop for INotifyWatcher { fn drop (& mut self) { self . channel . send (EventLoopMsg :: Shutdown) . unwrap () ; self . waker . wake () . unwrap () ; } }
};
}
