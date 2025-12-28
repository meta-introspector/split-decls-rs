macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        unsafe impl lock_api :: RawRwLockUpgrade for RawRwLock { # [inline] fn lock_upgradable (& self) { if ! self . try_lock_upgradable_fast () { let result = self . lock_upgradable_slow (None) ; debug_assert ! (result) ; } self . deadlock_acquire () ; } # [inline] fn try_lock_upgradable (& self) -> bool { let result = if self . try_lock_upgradable_fast () { true } else { self . try_lock_upgradable_slow () } ; if result { self . deadlock_acquire () ; } result } # [inline] unsafe fn unlock_upgradable (& self) { self . deadlock_release () ; let state = self . state . load (Ordering :: Relaxed) ; # [allow (clippy :: collapsible_if)] if state & PARKED_BIT == 0 { if self . state . compare_exchange_weak (state , state - (ONE_READER | UPGRADABLE_BIT) , Ordering :: Release , Ordering :: Relaxed ,) . is_ok () { return ; } } self . unlock_upgradable_slow (false) ; } # [inline] unsafe fn upgrade (& self) { let state = self . state . fetch_sub ((ONE_READER | UPGRADABLE_BIT) - WRITER_BIT , Ordering :: Acquire ,) ; if state & READERS_MASK != ONE_READER { let result = self . upgrade_slow (None) ; debug_assert ! (result) ; } } # [inline] unsafe fn try_upgrade (& self) -> bool { if self . state . compare_exchange_weak (ONE_READER | UPGRADABLE_BIT , WRITER_BIT , Ordering :: Acquire , Ordering :: Relaxed ,) . is_ok () { true } else { self . try_upgrade_slow () } } }
    };
}

impl_69!();