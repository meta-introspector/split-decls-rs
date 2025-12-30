// Generated macro for impl_228 (impl)
macro_rules! Depcrate_rwlockimpl_228 {
() => {
// Module: crate::rwlock
// Provides: {"impl_228"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Deref for MappedRwLockWriteGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
};
}
