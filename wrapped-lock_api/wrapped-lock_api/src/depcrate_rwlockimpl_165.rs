// Generated macro for impl_165 (impl)
macro_rules! Depcrate_rwlockimpl_165 {
() => {
// Module: crate::rwlock
// Provides: {"impl_165"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Deref for ArcRwLockReadGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
};
}
