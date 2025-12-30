// Generated macro for impl_218 (impl)
macro_rules! Depcrate_rwlockimpl_218 {
() => {
// Module: crate::rwlock
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Deref for MappedRwLockReadGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
};
}
