// Generated macro for impl_2644 (impl)
macro_rules! Depcrate_lock_muteximpl_2644 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2644"}
// Dependencies: {}
impl < T : ? Sized > Drop for MutexLockFuture < '_ , T > { fn drop (& mut self) { if let Some (mutex) = self . mutex { mutex . remove_waker (self . wait_key , true) ; } } }
};
}
