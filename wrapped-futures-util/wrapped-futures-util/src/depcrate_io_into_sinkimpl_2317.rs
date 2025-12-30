// Generated macro for impl_2317 (impl)
macro_rules! Depcrate_io_into_sinkimpl_2317 {
() => {
// Module: crate::io::into_sink
// Provides: {"impl_2317"}
// Dependencies: {}
impl < W : AsyncWrite , Item : AsRef < [u8] > > Sink < Item > for IntoSink < W , Item > { type Error = io :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . poll_flush_buffer (cx)) ? ; Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { debug_assert ! (self . buffer . is_none ()) ; * self . project () . buffer = Some (Block { offset : 0 , bytes : item }) ; Ok (()) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . poll_flush_buffer (cx)) ? ; ready ! (self . project () . writer . poll_flush (cx)) ? ; Poll :: Ready (Ok (())) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . poll_flush_buffer (cx)) ? ; ready ! (self . project () . writer . poll_close (cx)) ? ; Poll :: Ready (Ok (())) } }
};
}
