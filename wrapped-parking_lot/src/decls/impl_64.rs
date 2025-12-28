macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        unsafe impl lock_api :: RawRwLockFair for RawRwLock { # [inline] unsafe fn unlock_shared_fair (& self) { self . unlock_shared () ; } # [inline] unsafe fn unlock_exclusive_fair (& self) { self . deadlock_release () ; if self . state . compare_exchange (WRITER_BIT , 0 , Ordering :: Release , Ordering :: Relaxed) . is_ok () { return ; } self . unlock_exclusive_slow (true) ; } # [inline] unsafe fn bump_shared (& self) { if self . state . load (Ordering :: Relaxed) & WRITER_BIT != 0 { self . bump_shared_slow () ; } } # [inline] unsafe fn bump_exclusive (& self) { if self . state . load (Ordering :: Relaxed) & PARKED_BIT != 0 { self . bump_exclusive_slow () ; } } }
    };
}

impl_64!();