// Generated macro for impl_468 (impl)
macro_rules! Depcrate_sync_rwlockimpl_468 {
() => {
// Module: crate::sync::rwlock
// Provides: {"impl_468"}
// Dependencies: {}
impl < 'a , T > ops :: DerefMut for RwLockWriteGuard < 'a , T > { fn deref_mut (& mut self) -> & mut T { self . data . as_mut () . unwrap () . deref_mut () } }
};
}
