// Generated macro for impl_167 (impl)
macro_rules! Depcrate_rwlockimpl_167 {
() => {
// Module: crate::rwlock
// Provides: {"impl_167"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Debug + ? Sized > fmt :: Debug for ArcRwLockReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
