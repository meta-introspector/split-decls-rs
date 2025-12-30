// Generated macro for impl_467 (impl)
macro_rules! Depcrate_sync_rwlockimpl_467 {
() => {
// Module: crate::sync::rwlock
// Provides: {"impl_467"}
// Dependencies: {}
impl < 'a , T > ops :: Deref for RwLockWriteGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . data . as_ref () . unwrap () . deref () } }
};
}
