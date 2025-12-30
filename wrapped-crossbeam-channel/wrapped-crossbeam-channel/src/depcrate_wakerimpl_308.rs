// Generated macro for impl_308 (impl)
macro_rules! Depcrate_wakerimpl_308 {
() => {
// Module: crate::waker
// Provides: {"impl_308"}
// Dependencies: {}
impl Drop for SyncWaker { # [inline] fn drop (& mut self) { debug_assert ! (self . is_empty . load (Ordering :: SeqCst)) ; } }
};
}
