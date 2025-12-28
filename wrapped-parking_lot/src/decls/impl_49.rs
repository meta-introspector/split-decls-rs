macro_rules! deps {
    () => {
        RawMutex!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        unsafe impl lock_api :: RawMutexFair for RawMutex { # [inline] unsafe fn unlock_fair (& self) { # [cfg (feature = "deadlock_detection")] deadlock :: release_resource (self as * const _ as usize) ; if self . state . compare_exchange (LOCKED_BIT , 0 , Ordering :: Release , Ordering :: Relaxed) . is_ok () { return ; } self . unlock_slow (true) ; } # [inline] unsafe fn bump (& self) { if self . state . load (Ordering :: Relaxed) & PARKED_BIT != 0 { self . bump_slow () ; } } }
    };
}

impl_49!();