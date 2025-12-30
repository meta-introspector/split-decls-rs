// Generated macro for impl_158 (impl)
macro_rules! Depcrate_pollimpl_158 {
() => {
// Module: crate::poll
// Provides: {"impl_158"}
// Dependencies: {}
impl Drop for PollWatcher { fn drop (& mut self) { self . want_to_stop . store (true , Ordering :: Relaxed) ; } }
};
}
