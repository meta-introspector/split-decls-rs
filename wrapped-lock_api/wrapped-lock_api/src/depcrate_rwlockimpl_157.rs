// Generated macro for impl_157 (impl)
macro_rules! Depcrate_rwlockimpl_157 {
() => {
// Module: crate::rwlock
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Deref for RwLockReadGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
};
}
