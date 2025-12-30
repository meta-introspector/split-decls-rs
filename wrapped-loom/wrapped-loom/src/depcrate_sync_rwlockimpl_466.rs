// Generated macro for impl_466 (impl)
macro_rules! Depcrate_sync_rwlockimpl_466 {
() => {
// Module: crate::sync::rwlock
// Provides: {"impl_466"}
// Dependencies: {}
impl < 'a , T : 'a > Drop for RwLockReadGuard < 'a , T > { fn drop (& mut self) { self . data = None ; self . lock . object . release_read_lock () } }
};
}
