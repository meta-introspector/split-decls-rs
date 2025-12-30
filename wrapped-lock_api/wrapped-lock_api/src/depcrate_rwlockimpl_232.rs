// Generated macro for impl_232 (impl)
macro_rules! Depcrate_rwlockimpl_232 {
() => {
// Module: crate::rwlock
// Provides: {"impl_232"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedRwLockWriteGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
