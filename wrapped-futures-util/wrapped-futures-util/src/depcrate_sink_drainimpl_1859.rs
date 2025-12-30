// Generated macro for impl_1859 (impl)
macro_rules! Depcrate_sink_drainimpl_1859 {
() => {
// Module: crate::sink::drain
// Provides: {"impl_1859"}
// Dependencies: {}
impl < T > Sink < T > for Drain < T > { type Error = Infallible ; fn poll_ready (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , _item : T) -> Result < () , Self :: Error > { Ok (()) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } }
};
}
