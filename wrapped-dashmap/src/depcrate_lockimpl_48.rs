// Generated macro for impl_48 (impl)
macro_rules! Depcrate_lockimpl_48 {
() => {
// Module: crate::lock
// Provides: {"impl_48"}
// Dependencies: {}
unsafe impl lock_api :: RawRwLockDowngrade for RawRwLock { # [inline] unsafe fn downgrade (& self) { let state = self . state . fetch_and (ONE_READER | WRITERS_PARKED , Ordering :: Release) ; if state & READERS_PARKED != 0 { parking_lot_core :: unpark_all ((self as * const _ as usize) + 1 , UnparkToken (0)) ; } } }
};
}
