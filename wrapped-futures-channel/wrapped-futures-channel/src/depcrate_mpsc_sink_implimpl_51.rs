// Generated macro for impl_51 (impl)
macro_rules! Depcrate_mpsc_sink_implimpl_51 {
() => {
// Module: crate::mpsc::sink_impl
// Provides: {"impl_51"}
// Dependencies: {}
impl < T > Sink < T > for UnboundedSender < T > { type Error = SendError ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Self :: poll_ready (& * self , cx) } fn start_send (mut self : Pin < & mut Self > , msg : T) -> Result < () , Self :: Error > { Self :: start_send (& mut * self , msg) } fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn poll_close (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . disconnect () ; Poll :: Ready (Ok (())) } }
};
}
