// Generated macro for impl_221 (impl)
macro_rules! Depcrate_rwlockimpl_221 {
() => {
// Module: crate::rwlock
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for MappedRwLockReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
