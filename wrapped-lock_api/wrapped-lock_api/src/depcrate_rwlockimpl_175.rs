// Generated macro for impl_175 (impl)
macro_rules! Depcrate_rwlockimpl_175 {
() => {
// Module: crate::rwlock
// Provides: {"impl_175"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Deref for RwLockWriteGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
};
}
