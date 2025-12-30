// Generated macro for impl_179 (impl)
macro_rules! Depcrate_rwlockimpl_179 {
() => {
// Module: crate::rwlock
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for RwLockWriteGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
