// Generated macro for impl_166 (impl)
macro_rules! Depcrate_rwlockimpl_166 {
() => {
// Module: crate::rwlock
// Provides: {"impl_166"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Drop for ArcRwLockReadGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_shared () ; } } }
};
}
