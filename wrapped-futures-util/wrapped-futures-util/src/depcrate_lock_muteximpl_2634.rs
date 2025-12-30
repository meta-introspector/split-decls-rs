// Generated macro for impl_2634 (impl)
macro_rules! Depcrate_lock_muteximpl_2634 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2634"}
// Dependencies: {}
impl < T : ? Sized > Drop for OwnedMutexLockFuture < T > { fn drop (& mut self) { if let Some (mutex) = self . mutex . as_ref () { mutex . remove_waker (self . wait_key , true) ; } } }
};
}
