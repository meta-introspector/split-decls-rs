macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        unsafe impl lock_api :: RawRwLockDowngrade for RawRwLock { # [inline] unsafe fn downgrade (& self) { let state = self . state . fetch_add (ONE_READER - WRITER_BIT , Ordering :: Release) ; if state & PARKED_BIT != 0 { self . downgrade_slow () ; } } }
    };
}

impl_65!();