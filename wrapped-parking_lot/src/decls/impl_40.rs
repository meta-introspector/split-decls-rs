macro_rules! deps {
    () => {
        RawFairMutex!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        unsafe impl lock_api :: RawMutexFair for RawFairMutex { # [inline] unsafe fn unlock_fair (& self) { self . 0 . unlock_fair () } # [inline] unsafe fn bump (& self) { self . 0 . bump () } }
    };
}

impl_40!();