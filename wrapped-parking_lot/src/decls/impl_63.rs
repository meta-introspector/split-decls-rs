macro_rules! deps {
    () => {
        RawRwLock!();
        GuardMarker!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        unsafe impl lock_api :: RawRwLock for RawRwLock { const INIT : RawRwLock = RawRwLock { state : AtomicUsize :: new (0) , } ; type GuardMarker = crate :: GuardMarker ; # [inline] fn lock_exclusive (& self) { if self . state . compare_exchange_weak (0 , WRITER_BIT , Ordering :: Acquire , Ordering :: Relaxed) . is_err () { let result = self . lock_exclusive_slow (None) ; debug_assert ! (result) ; } self . deadlock_acquire () ; } # [inline] fn try_lock_exclusive (& self) -> bool { if self . state . compare_exchange (0 , WRITER_BIT , Ordering :: Acquire , Ordering :: Relaxed) . is_ok () { self . deadlock_acquire () ; true } else { false } } # [inline] unsafe fn unlock_exclusive (& self) { self . deadlock_release () ; if self . state . compare_exchange (WRITER_BIT , 0 , Ordering :: Release , Ordering :: Relaxed) . is_ok () { return ; } self . unlock_exclusive_slow (false) ; } # [inline] fn lock_shared (& self) { if ! self . try_lock_shared_fast (false) { let result = self . lock_shared_slow (false , None) ; debug_assert ! (result) ; } self . deadlock_acquire () ; } # [inline] fn try_lock_shared (& self) -> bool { let result = if self . try_lock_shared_fast (false) { true } else { self . try_lock_shared_slow (false) } ; if result { self . deadlock_acquire () ; } result } # [inline] unsafe fn unlock_shared (& self) { self . deadlock_release () ; let state = if have_elision () { self . state . elision_fetch_sub_release (ONE_READER) } else { self . state . fetch_sub (ONE_READER , Ordering :: Release) } ; if state & (READERS_MASK | WRITER_PARKED_BIT) == (ONE_READER | WRITER_PARKED_BIT) { self . unlock_shared_slow () ; } } # [inline] fn is_locked (& self) -> bool { let state = self . state . load (Ordering :: Relaxed) ; state & (WRITER_BIT | READERS_MASK) != 0 } # [inline] fn is_locked_exclusive (& self) -> bool { let state = self . state . load (Ordering :: Relaxed) ; state & (WRITER_BIT) != 0 } }
    };
}

impl_63!();