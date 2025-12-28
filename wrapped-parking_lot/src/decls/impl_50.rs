macro_rules! deps {
    () => {
        RawMutex!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        unsafe impl lock_api :: RawMutexTimed for RawMutex { type Duration = Duration ; type Instant = Instant ; # [inline] fn try_lock_until (& self , timeout : Instant) -> bool { let result = if self . state . compare_exchange_weak (0 , LOCKED_BIT , Ordering :: Acquire , Ordering :: Relaxed) . is_ok () { true } else { self . lock_slow (Some (timeout)) } ; if result { # [cfg (feature = "deadlock_detection")] unsafe { deadlock :: acquire_resource (self as * const _ as usize) } ; } result } # [inline] fn try_lock_for (& self , timeout : Duration) -> bool { let result = if self . state . compare_exchange_weak (0 , LOCKED_BIT , Ordering :: Acquire , Ordering :: Relaxed) . is_ok () { true } else { self . lock_slow (util :: to_deadline (timeout)) } ; if result { # [cfg (feature = "deadlock_detection")] unsafe { deadlock :: acquire_resource (self as * const _ as usize) } ; } result } }
    };
}

impl_50!()