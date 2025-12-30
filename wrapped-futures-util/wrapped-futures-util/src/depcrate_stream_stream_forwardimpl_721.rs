// Generated macro for impl_721 (impl)
macro_rules! Depcrate_stream_stream_forwardimpl_721 {
() => {
// Module: crate::stream::stream::forward
// Provides: {"impl_721"}
// Dependencies: {}
impl < St , Si , Item , E > Future for Forward < St , Si , Item > where Si : Sink < Item , Error = E > , St : Stream < Item = Item > , { type Output = Result < () , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let ForwardProj { mut sink , mut stream , buffered_item } = self . project () ; let mut si = sink . as_mut () . as_pin_mut () . expect ("polled `Forward` after completion") ; loop { if buffered_item . is_some () { ready ! (si . as_mut () . poll_ready (cx)) ? ; si . as_mut () . start_send (buffered_item . take () . unwrap ()) ? ; } match stream . as_mut () . poll_next (cx) { Poll :: Ready (Some (item)) => { * buffered_item = Some (item) ; } Poll :: Ready (None) => { ready ! (si . poll_close (cx)) ? ; sink . set (None) ; return Poll :: Ready (Ok (())) ; } Poll :: Pending => { ready ! (si . poll_flush (cx)) ? ; return Poll :: Pending ; } } } } }
};
}
