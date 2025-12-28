macro_rules! deps {
    () => {
        RawRwLock!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        unsafe impl lock_api :: RawRwLockDowngrade for RawRwLock { # [inline] unsafe fn downgrade (& self) { let state = self . state . fetch_and (ONE_READER | WRITERS_PARKED , Ordering :: Release) ; if state & READERS_PARKED != 0 { parking_lot_core :: unpark_all ((self as * const _ as usize) + 1 , UnparkToken (0)) ; } } }
    };
}

impl_33!();