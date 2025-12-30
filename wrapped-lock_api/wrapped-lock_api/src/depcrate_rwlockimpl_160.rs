// Generated macro for impl_160 (impl)
macro_rules! Depcrate_rwlockimpl_160 {
() => {
// Module: crate::rwlock
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for RwLockReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
