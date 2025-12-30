// Generated macro for impl_97 (impl)
macro_rules! Depcrate_raw_rwlockimpl_97 {
() => {
// Module: crate::raw_rwlock
// Provides: {"impl_97"}
// Dependencies: {}
unsafe impl lock_api :: RawRwLockTimed for RawRwLock { type Duration = Duration ; type Instant = Instant ; # [inline] fn try_lock_shared_for (& self , timeout : Self :: Duration) -> bool { let result = if self . try_lock_shared_fast (false) { true } else { self . lock_shared_slow (false , util :: to_deadline (timeout)) } ; if result { self . deadlock_acquire () ; } result } # [inline] fn try_lock_shared_until (& self , timeout : Self :: Instant) -> bool { let result = if self . try_lock_shared_fast (false) { true } else { self . lock_shared_slow (false , Some (timeout)) } ; if result { self . deadlock_acquire () ; } result } # [inline] fn try_lock_exclusive_for (& self , timeout : Duration) -> bool { let result = if self . state . compare_exchange_weak (0 , WRITER_BIT , Ordering :: Acquire , Ordering :: Relaxed) . is_ok () { true } else { self . lock_exclusive_slow (util :: to_deadline (timeout)) } ; if result { self . deadlock_acquire () ; } result } # [inline] fn try_lock_exclusive_until (& self , timeout : Instant) -> bool { let result = if self . state . compare_exchange_weak (0 , WRITER_BIT , Ordering :: Acquire , Ordering :: Relaxed) . is_ok () { true } else { self . lock_exclusive_slow (Some (timeout)) } ; if result { self . deadlock_acquire () ; } result } }
};
}
