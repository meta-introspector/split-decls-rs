// Generated macro for impl_1942 (impl)
macro_rules! Depcrate_sink_send_allimpl_1942 {
() => {
// Module: crate::sink::send_all
// Provides: {"impl_1942"}
// Dependencies: {}
impl < Si , St , Ok , Error > Future for SendAll < '_ , Si , St > where Si : Sink < Ok , Error = Error > + Unpin + ? Sized , St : Stream < Item = Result < Ok , Error > > , { type Output = Result < () , Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { if let Some (item) = self . as_mut () . project () . buffered . take () { ready ! (self . as_mut () . try_start_send (cx , item)) ? } loop { let this = self . as_mut () . project () ; match this . stream . try_poll_next (cx) ? { Poll :: Ready (Some (item)) => ready ! (self . as_mut () . try_start_send (cx , item)) ? , Poll :: Ready (None) => { ready ! (Pin :: new (this . sink) . poll_flush (cx)) ? ; return Poll :: Ready (Ok (())) ; } Poll :: Pending => { ready ! (Pin :: new (this . sink) . poll_flush (cx)) ? ; return Poll :: Pending ; } } } } }
};
}
