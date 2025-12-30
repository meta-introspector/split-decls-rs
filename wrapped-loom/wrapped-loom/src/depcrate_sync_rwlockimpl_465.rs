// Generated macro for impl_465 (impl)
macro_rules! Depcrate_sync_rwlockimpl_465 {
() => {
// Module: crate::sync::rwlock
// Provides: {"impl_465"}
// Dependencies: {}
impl < 'a , T > ops :: Deref for RwLockReadGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . data . as_ref () . unwrap () . deref () } }
};
}
