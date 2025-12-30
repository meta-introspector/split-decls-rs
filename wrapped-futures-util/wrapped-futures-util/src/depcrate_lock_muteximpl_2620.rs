// Generated macro for impl_2620 (impl)
macro_rules! Depcrate_lock_muteximpl_2620 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2620"}
// Dependencies: {}
impl < T : ? Sized > fmt :: Debug for Mutex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let state = self . state . load (Ordering :: SeqCst) ; f . debug_struct ("Mutex") . field ("is_locked" , & ((state & IS_LOCKED) != 0)) . field ("has_waiters" , & ((state & HAS_WAITERS) != 0)) . finish () } }
};
}
