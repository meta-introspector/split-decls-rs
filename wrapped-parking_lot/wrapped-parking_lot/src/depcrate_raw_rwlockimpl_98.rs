// Generated macro for impl_98 (impl)
macro_rules! Depcrate_raw_rwlockimpl_98 {
() => {
// Module: crate::raw_rwlock
// Provides: {"impl_98"}
// Dependencies: {}
unsafe impl lock_api :: RawRwLockRecursive for RawRwLock { # [inline] fn lock_shared_recursive (& self) { if ! self . try_lock_shared_fast (true) { let result = self . lock_shared_slow (true , None) ; debug_assert ! (result) ; } self . deadlock_acquire () ; } # [inline] fn try_lock_shared_recursive (& self) -> bool { let result = if self . try_lock_shared_fast (true) { true } else { self . try_lock_shared_slow (true) } ; if result { self . deadlock_acquire () ; } result } }
};
}
