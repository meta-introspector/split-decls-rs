// Generated macro for impl_61 (impl)
macro_rules! Depcrate_address_channelimpl_61 {
() => {
// Module: crate::address::channel
// Provides: {"impl_61"}
// Dependencies: {}
impl < A : Actor > Drop for AddressReceiver < A > { fn drop (& mut self) { self . inner . set_closed () ; while let Some (task) = unsafe { self . inner . parked_queue . pop_spin () } { task . lock () . notify () ; } loop { match self . next_message () { Poll :: Ready (Some (_)) => { } Poll :: Ready (None) => break , Poll :: Pending => { let state = decode_state (self . inner . state . load (SeqCst)) ; if state . is_closed () { break ; } thread :: yield_now () ; } } } } }
};
}
