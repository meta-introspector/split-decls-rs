macro_rules! deps {
    () => {
        RawMutex!();
        GuardMarker!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        unsafe impl lock_api :: RawMutex for RawMutex { const INIT : RawMutex = RawMutex { state : AtomicU8 :: new (0) , } ; type GuardMarker = crate :: GuardMarker ; # [inline] fn lock (& self) { if self . state . compare_exchange_weak (0 , LOCKED_BIT , Ordering :: Acquire , Ordering :: Relaxed) . is_err () { self . lock_slow (None) ; } # [cfg (feature = "deadlock_detection")] unsafe { deadlock :: acquire_resource (self as * const _ as usize) } ; } # [inline] fn try_lock (& self) -> bool { let mut state = self . state . load (Ordering :: Relaxed) ; loop { if state & LOCKED_BIT != 0 { return false ; } match self . state . compare_exchange_weak (state , state | LOCKED_BIT , Ordering :: Acquire , Ordering :: Relaxed ,) { Ok (_) => { # [cfg (feature = "deadlock_detection")] unsafe { deadlock :: acquire_resource (self as * const _ as usize) } ; return true ; } Err (x) => state = x , } } } # [inline] unsafe fn unlock (& self) { # [cfg (feature = "deadlock_detection")] deadlock :: release_resource (self as * const _ as usize) ; if self . state . compare_exchange (LOCKED_BIT , 0 , Ordering :: Release , Ordering :: Relaxed) . is_ok () { return ; } self . unlock_slow (false) ; } # [inline] fn is_locked (& self) -> bool { let state = self . state . load (Ordering :: Relaxed) ; state & LOCKED_BIT != 0 } }
    };
}

impl_48!()