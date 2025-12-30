// Generated macro for impl_96 (impl)
macro_rules! Depcrate_raw_rwlockimpl_96 {
() => {
// Module: crate::raw_rwlock
// Provides: {"impl_96"}
// Dependencies: {}
unsafe impl lock_api :: RawRwLockDowngrade for RawRwLock { # [inline] unsafe fn downgrade (& self) { let state = self . state . fetch_add (ONE_READER - WRITER_BIT , Ordering :: Release) ; if state & PARKED_BIT != 0 { self . downgrade_slow () ; } } }
};
}
