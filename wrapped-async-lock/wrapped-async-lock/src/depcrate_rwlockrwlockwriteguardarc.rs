// Generated macro for RwLockWriteGuardArc (struct)
macro_rules! Depcrate_rwlockRwLockWriteGuardArc {
() => {
// Module: crate::rwlock
// Provides: {"RwLockWriteGuardArc"}
// Dependencies: {}
# [doc = " An owned, reference-counted guard that releases the write lock when dropped."] # [clippy :: has_significant_drop] pub struct RwLockWriteGuardArc < T : ? Sized > { lock : Arc < RwLock < T > > , }
};
}
