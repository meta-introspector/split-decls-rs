// Generated macro for impl_15 (impl)
macro_rules! Depcrate_mpscimpl_15 {
() => {
// Module: crate::mpsc
// Provides: {"impl_15"}
// Dependencies: {}
impl < T > Sink < T > for Sender < T > { type Error = SendError < T > ; fn poll_ready (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , item : T) -> Result < () , SendError < T > > { self . send (item) } fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , SendError < T > > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } }
};
}
