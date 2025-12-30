// Generated macro for impl_188 (impl)
macro_rules! Depcrate_rwlockimpl_188 {
() => {
// Module: crate::rwlock
// Provides: {"impl_188"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Drop for ArcRwLockWriteGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_exclusive () ; } } }
};
}
