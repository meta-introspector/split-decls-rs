macro_rules! deps {
    () => {
        RawReentrantMutex!();
        RawMutexTimed!();
        GetThreadId!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < R : RawMutexTimed , G : GetThreadId > RawReentrantMutex < R , G > { # [doc = " Attempts to acquire this lock until a timeout is reached."] # [inline] pub fn try_lock_until (& self , timeout : R :: Instant) -> bool { self . lock_internal (| | self . mutex . try_lock_until (timeout)) } # [doc = " Attempts to acquire this lock until a timeout is reached."] # [inline] pub fn try_lock_for (& self , timeout : R :: Duration) -> bool { self . lock_internal (| | self . mutex . try_lock_for (timeout)) } }
    };
}

impl_57!();