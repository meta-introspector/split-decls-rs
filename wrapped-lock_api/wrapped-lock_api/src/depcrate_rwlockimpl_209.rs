// Generated macro for impl_209 (impl)
macro_rules! Depcrate_rwlockimpl_209 {
() => {
// Module: crate::rwlock
// Provides: {"impl_209"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : ? Sized > Deref for ArcRwLockUpgradableReadGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
};
}
