// Generated macro for impl_2648 (impl)
macro_rules! Depcrate_lock_muteximpl_2648 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2648"}
// Dependencies: {}
impl < T : ? Sized > Drop for MutexGuard < '_ , T > { fn drop (& mut self) { self . mutex . unlock () } }
};
}
