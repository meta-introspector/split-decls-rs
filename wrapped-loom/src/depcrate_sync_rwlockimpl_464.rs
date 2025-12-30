// Generated macro for impl_464 (impl)
macro_rules! Depcrate_sync_rwlockimpl_464 {
() => {
// Module: crate::sync::rwlock
// Provides: {"impl_464"}
// Dependencies: {}
impl < T > From < T > for RwLock < T > { # [doc = " Creates a new rwlock in an unlocked state ready for use."] # [doc = " This is equivalent to [`RwLock::new`]."] fn from (t : T) -> Self { Self :: new (t) } }
};
}
