// Generated macro for maybe_cancel (function)
macro_rules! Depcrate_proto_streams_streamsmaybe_cancel {
() => {
// Module: crate::proto::streams::streams
// Provides: {"maybe_cancel"}
// Dependencies: {}
fn maybe_cancel (stream : & mut store :: Ptr , actions : & mut Actions , counts : & mut Counts) { if stream . is_canceled_interest () { let reason = if counts . peer () . is_server () && stream . state . is_send_closed () && stream . state . is_recv_streaming () { Reason :: NO_ERROR } else { Reason :: CANCEL } ; actions . send . schedule_implicit_reset (stream , reason , counts , & mut actions . task) ; actions . recv . enqueue_reset_expiration (stream , counts) ; } }
};
}
