macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        unsafe impl lock_api :: RawRwLockUpgradeTimed for RawRwLock { # [inline] fn try_lock_upgradable_until (& self , timeout : Instant) -> bool { let result = if self . try_lock_upgradable_fast () { true } else { self . lock_upgradable_slow (Some (timeout)) } ; if result { self . deadlock_acquire () ; } result } # [inline] fn try_lock_upgradable_for (& self , timeout : Duration) -> bool { let result = if self . try_lock_upgradable_fast () { true } else { self . lock_upgradable_slow (util :: to_deadline (timeout)) } ; if result { self . deadlock_acquire () ; } result } # [inline] unsafe fn try_upgrade_until (& self , timeout : Instant) -> bool { let state = self . state . fetch_sub ((ONE_READER | UPGRADABLE_BIT) - WRITER_BIT , Ordering :: Relaxed ,) ; if state & READERS_MASK == ONE_READER { true } else { self . upgrade_slow (Some (timeout)) } } # [inline] unsafe fn try_upgrade_for (& self , timeout : Duration) -> bool { let state = self . state . fetch_sub ((ONE_READER | UPGRADABLE_BIT) - WRITER_BIT , Ordering :: Relaxed ,) ; if state & READERS_MASK == ONE_READER { true } else { self . upgrade_slow (util :: to_deadline (timeout)) } } }
    };
}

impl_72!();