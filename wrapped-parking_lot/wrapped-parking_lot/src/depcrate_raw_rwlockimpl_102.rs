// Generated macro for impl_102 (impl)
macro_rules! Depcrate_raw_rwlockimpl_102 {
() => {
// Module: crate::raw_rwlock
// Provides: {"impl_102"}
// Dependencies: {}
unsafe impl lock_api :: RawRwLockUpgradeDowngrade for RawRwLock { # [inline] unsafe fn downgrade_upgradable (& self) { let state = self . state . fetch_sub (UPGRADABLE_BIT , Ordering :: Relaxed) ; if state & PARKED_BIT != 0 { self . downgrade_slow () ; } } # [inline] unsafe fn downgrade_to_upgradable (& self) { let state = self . state . fetch_add ((ONE_READER | UPGRADABLE_BIT) - WRITER_BIT , Ordering :: Release ,) ; if state & PARKED_BIT != 0 { self . downgrade_to_upgradable_slow () ; } } }
};
}
