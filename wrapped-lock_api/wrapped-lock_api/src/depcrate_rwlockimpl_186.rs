// Generated macro for impl_186 (impl)
macro_rules! Depcrate_rwlockimpl_186 {
() => {
// Module: crate::rwlock
// Provides: {"impl_186"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Deref for ArcRwLockWriteGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
};
}
