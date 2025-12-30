// Generated macro for impl_62 (impl)
macro_rules! Depcrate_address_channelimpl_62 {
() => {
// Module: crate::address::channel
// Provides: {"impl_62"}
// Dependencies: {}
impl < A : Actor > Inner < A > { fn max_senders (& self) -> usize { MAX_CAPACITY - self . buffer . load (Relaxed) } fn set_closed (& self) { let curr = self . state . load (SeqCst) ; if ! decode_state (curr) . is_open { return ; } self . state . fetch_and (! OPEN_MASK , SeqCst) ; } }
};
}
