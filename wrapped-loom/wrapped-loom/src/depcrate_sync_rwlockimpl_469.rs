// Generated macro for impl_469 (impl)
macro_rules! Depcrate_sync_rwlockimpl_469 {
() => {
// Module: crate::sync::rwlock
// Provides: {"impl_469"}
// Dependencies: {}
impl < 'a , T : 'a > Drop for RwLockWriteGuard < 'a , T > { fn drop (& mut self) { self . data = None ; self . lock . object . release_write_lock () } }
};
}
