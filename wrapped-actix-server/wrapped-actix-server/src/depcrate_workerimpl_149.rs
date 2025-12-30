// Generated macro for impl_149 (impl)
macro_rules! Depcrate_workerimpl_149 {
() => {
// Module: crate::worker
// Provides: {"impl_149"}
// Dependencies: {}
impl Drop for WorkerCounterGuard { fn drop (& mut self) { let (waker_queue , counter) = & * self . 0 . inner ; if counter . dec () { waker_queue . wake (WakerInterest :: WorkerAvailable (self . 0 . idx)) ; } } }
};
}
