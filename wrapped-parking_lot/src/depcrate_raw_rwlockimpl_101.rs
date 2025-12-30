// Generated macro for impl_101 (impl)
macro_rules! Depcrate_raw_rwlockimpl_101 {
() => {
// Module: crate::raw_rwlock
// Provides: {"impl_101"}
// Dependencies: {}
unsafe impl lock_api :: RawRwLockUpgradeFair for RawRwLock { # [inline] unsafe fn unlock_upgradable_fair (& self) { self . deadlock_release () ; let state = self . state . load (Ordering :: Relaxed) ; # [allow (clippy :: collapsible_if)] if state & PARKED_BIT == 0 { if self . state . compare_exchange_weak (state , state - (ONE_READER | UPGRADABLE_BIT) , Ordering :: Release , Ordering :: Relaxed ,) . is_ok () { return ; } } self . unlock_upgradable_slow (false) ; } # [inline] unsafe fn bump_upgradable (& self) { if self . state . load (Ordering :: Relaxed) & PARKED_BIT != 0 { self . bump_upgradable_slow () ; } } }
};
}
