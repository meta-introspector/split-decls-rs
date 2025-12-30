// Generated macro for impl_220 (impl)
macro_rules! Depcrate_rwlockimpl_220 {
() => {
// Module: crate::rwlock
// Provides: {"impl_220"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedRwLockReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
