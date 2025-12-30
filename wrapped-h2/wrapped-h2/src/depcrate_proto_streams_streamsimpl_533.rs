// Generated macro for impl_533 (impl)
macro_rules! Depcrate_proto_streams_streamsimpl_533 {
() => {
// Module: crate::proto::streams::streams
// Provides: {"impl_533"}
// Dependencies: {}
impl < B > Streams < B , client :: Peer > where B : Buf , { pub fn poll_pending_open (& mut self , cx : & Context , pending : Option < & OpaqueStreamRef > ,) -> Poll < Result < () , crate :: Error > > { let mut me = self . inner . lock () . unwrap () ; let me = & mut * me ; me . actions . ensure_no_conn_error () ? ; me . actions . send . ensure_next_stream_id () ? ; if let Some (pending) = pending { let mut stream = me . store . resolve (pending . key) ; tracing :: trace ! ("poll_pending_open; stream = {:?}" , stream . is_pending_open) ; if stream . is_pending_open { stream . wait_send (cx) ; return Poll :: Pending ; } } Poll :: Ready (Ok (())) } }
};
}
