// Generated macro for impl_60 (impl)
macro_rules! Depcrate_address_channelimpl_60 {
() => {
// Module: crate::address::channel
// Provides: {"impl_60"}
// Dependencies: {}
impl < A : Actor > Stream for AddressReceiver < A > { type Item = Envelope < A > ; fn poll_next (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . get_mut () ; match this . next_message () { Poll :: Ready (msg) => Poll :: Ready (msg) , Poll :: Pending => { this . inner . recv_task . register (cx . waker ()) ; this . next_message () } } } }
};
}
