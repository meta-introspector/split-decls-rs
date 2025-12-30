// Generated macro for impl_231 (impl)
macro_rules! Depcrate_rwlockimpl_231 {
() => {
// Module: crate::rwlock
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for MappedRwLockWriteGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
