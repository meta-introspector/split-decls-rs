// Generated macro for impl_189 (impl)
macro_rules! Depcrate_rwlockimpl_189 {
() => {
// Module: crate::rwlock
// Provides: {"impl_189"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Debug + ? Sized > fmt :: Debug for ArcRwLockWriteGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
