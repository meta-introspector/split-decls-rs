// Generated macro for impl_2003 (impl)
macro_rules! Depcrate_sink_bufferimpl_2003 {
() => {
// Module: crate::sink::buffer
// Provides: {"impl_2003"}
// Dependencies: {}
impl < Si : Sink < Item > , Item > Sink < Item > for Buffer < Si , Item > { type Error = Si :: Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if self . capacity == 0 { return self . project () . sink . poll_ready (cx) ; } let _ = self . as_mut () . try_empty_buffer (cx) ? ; if self . buf . len () >= self . capacity { Poll :: Pending } else { Poll :: Ready (Ok (())) } } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { if self . capacity == 0 { self . project () . sink . start_send (item) } else { self . project () . buf . push_back (item) ; Ok (()) } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . try_empty_buffer (cx)) ? ; debug_assert ! (self . buf . is_empty ()) ; self . project () . sink . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . as_mut () . try_empty_buffer (cx)) ? ; debug_assert ! (self . buf . is_empty ()) ; self . project () . sink . poll_close (cx) } }
};
}
