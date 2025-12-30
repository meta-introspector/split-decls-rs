// Generated macro for impl_211 (impl)
macro_rules! Depcrate_rwlockimpl_211 {
() => {
// Module: crate::rwlock
// Provides: {"impl_211"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : fmt :: Debug + ? Sized > fmt :: Debug for ArcRwLockUpgradableReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
