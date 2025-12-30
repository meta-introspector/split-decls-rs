// Generated macro for impl_447 (impl)
macro_rules! Depcrate_sync_muteximpl_447 {
() => {
// Module: crate::sync::mutex
// Provides: {"impl_447"}
// Dependencies: {}
impl < 'a , T : ? Sized + 'a > Drop for MutexGuard < 'a , T > { fn drop (& mut self) { self . data = None ; self . lock . object . release_lock () ; } }
};
}
