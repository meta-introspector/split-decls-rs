// Generated macro for impl_187 (impl)
macro_rules! Depcrate_rwlockimpl_187 {
() => {
// Module: crate::rwlock
// Provides: {"impl_187"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > DerefMut for ArcRwLockWriteGuard < R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . rwlock . data . get () } } }
};
}
