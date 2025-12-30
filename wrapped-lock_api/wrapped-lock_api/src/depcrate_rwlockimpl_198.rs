// Generated macro for impl_198 (impl)
macro_rules! Depcrate_rwlockimpl_198 {
() => {
// Module: crate::rwlock
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'a , R : RawRwLockUpgrade + 'a , T : ? Sized + 'a > Deref for RwLockUpgradableReadGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
};
}
