// Generated macro for impl_52 (impl)
macro_rules! Depcrate_mpsc_sink_implimpl_52 {
() => {
// Module: crate::mpsc::sink_impl
// Provides: {"impl_52"}
// Dependencies: {}
impl < T > Sink < T > for & UnboundedSender < T > { type Error = SendError ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { UnboundedSender :: poll_ready (* self , cx) } fn start_send (self : Pin < & mut Self > , msg : T) -> Result < () , Self :: Error > { self . unbounded_send (msg) . map_err (TrySendError :: into_send_error) } fn poll_flush (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . close_channel () ; Poll :: Ready (Ok (())) } }
};
}
