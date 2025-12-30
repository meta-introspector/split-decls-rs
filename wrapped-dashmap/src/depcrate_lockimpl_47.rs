// Generated macro for impl_47 (impl)
macro_rules! Depcrate_lockimpl_47 {
() => {
// Module: crate::lock
// Provides: {"impl_47"}
// Dependencies: {}
unsafe impl lock_api :: RawRwLock for RawRwLock { # [allow (clippy :: declare_interior_mutable_const)] const INIT : Self = Self { state : AtomicUsize :: new (0) , } ; type GuardMarker = lock_api :: GuardSend ; # [inline] fn try_lock_exclusive (& self) -> bool { self . state . compare_exchange (0 , ONE_WRITER , Ordering :: Acquire , Ordering :: Relaxed) . is_ok () } # [inline] fn lock_exclusive (& self) { if self . state . compare_exchange_weak (0 , ONE_WRITER , Ordering :: Acquire , Ordering :: Relaxed) . is_err () { self . lock_exclusive_slow () ; } } # [inline] unsafe fn unlock_exclusive (& self) { if self . state . compare_exchange (ONE_WRITER , 0 , Ordering :: Release , Ordering :: Relaxed) . is_err () { self . unlock_exclusive_slow () ; } } # [inline] fn try_lock_shared (& self) -> bool { self . try_lock_shared_fast () || self . try_lock_shared_slow () } # [inline] fn lock_shared (& self) { if ! self . try_lock_shared_fast () { self . lock_shared_slow () ; } } # [inline] unsafe fn unlock_shared (& self) { let state = self . state . fetch_sub (ONE_READER , Ordering :: Release) ; if state == (ONE_READER | WRITERS_PARKED) { self . unlock_shared_slow () ; } } }
};
}
