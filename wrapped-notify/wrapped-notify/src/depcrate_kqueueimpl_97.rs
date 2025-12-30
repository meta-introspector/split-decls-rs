// Generated macro for impl_97 (impl)
macro_rules! Depcrate_kqueueimpl_97 {
() => {
// Module: crate::kqueue
// Provides: {"impl_97"}
// Dependencies: {}
impl Drop for KqueueWatcher { fn drop (& mut self) { self . channel . send (EventLoopMsg :: Shutdown) . unwrap () ; self . waker . wake () . unwrap () ; } }
};
}
