macro_rules! deps {
    () => {
        RawFairMutex!();
        RawMutex!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        unsafe impl lock_api :: RawMutexTimed for RawFairMutex { type Duration = < RawMutex as lock_api :: RawMutexTimed > :: Duration ; type Instant = < RawMutex as lock_api :: RawMutexTimed > :: Instant ; # [inline] fn try_lock_until (& self , timeout : Self :: Instant) -> bool { self . 0 . try_lock_until (timeout) } # [inline] fn try_lock_for (& self , timeout : Self :: Duration) -> bool { self . 0 . try_lock_for (timeout) } }
    };
}

impl_41!()