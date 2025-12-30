// Generated macro for impl_52 (impl)
macro_rules! Depcrate_address_channelimpl_52 {
() => {
// Module: crate::address::channel
// Provides: {"impl_52"}
// Dependencies: {}
impl < A : Actor > Drop for AddressSender < A > { fn drop (& mut self) { let prev = self . inner . num_senders . fetch_sub (1 , SeqCst) ; if prev == 1 { self . inner . recv_task . wake () ; } } }
};
}
