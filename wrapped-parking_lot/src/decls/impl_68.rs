macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        unsafe impl lock_api :: RawRwLockRecursiveTimed for RawRwLock { # [inline] fn try_lock_shared_recursive_for (& self , timeout : Self :: Duration) -> bool { let result = if self . try_lock_shared_fast (true) { true } else { self . lock_shared_slow (true , util :: to_deadline (timeout)) } ; if result { self . deadlock_acquire () ; } result } # [inline] fn try_lock_shared_recursive_until (& self , timeout : Self :: Instant) -> bool { let result = if self . try_lock_shared_fast (true) { true } else { self . lock_shared_slow (true , Some (timeout)) } ; if result { self . deadlock_acquire () ; } result } }
    };
}

impl_68!()