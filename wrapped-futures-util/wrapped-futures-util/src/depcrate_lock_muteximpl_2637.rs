// Generated macro for impl_2637 (impl)
macro_rules! Depcrate_lock_muteximpl_2637 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2637"}
// Dependencies: {}
impl < T : ? Sized > Drop for OwnedMutexGuard < T > { fn drop (& mut self) { self . mutex . unlock () } }
};
}
