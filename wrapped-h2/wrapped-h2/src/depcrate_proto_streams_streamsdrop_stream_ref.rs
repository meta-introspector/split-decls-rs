// Generated macro for drop_stream_ref (function)
macro_rules! Depcrate_proto_streams_streamsdrop_stream_ref {
() => {
// Module: crate::proto::streams::streams
// Provides: {"drop_stream_ref"}
// Dependencies: {}
fn drop_stream_ref (inner : & Mutex < Inner > , key : store :: Key) { let mut me = match inner . lock () { Ok (inner) => inner , Err (_) => { if :: std :: thread :: panicking () { tracing :: trace ! ("StreamRef::drop; mutex poisoned") ; return ; } else { panic ! ("StreamRef::drop; mutex poisoned") ; } } } ; let me = & mut * me ; me . refs -= 1 ; let mut stream = me . store . resolve (key) ; tracing :: trace ! ("drop_stream_ref; stream={:?}" , stream) ; stream . ref_dec () ; let actions = & mut me . actions ; if stream . ref_count == 0 && stream . is_closed () { if let Some (task) = actions . task . take () { task . wake () ; } } me . counts . transition (stream , | counts , stream | { maybe_cancel (stream , actions , counts) ; if stream . ref_count == 0 { actions . recv . release_closed_capacity (stream , & mut actions . task) ; let mut ppp = stream . pending_push_promises . take () ; while let Some (promise) = ppp . pop (stream . store_mut ()) { counts . transition (promise , | counts , stream | { maybe_cancel (stream , actions , counts) ; }) ; } } }) ; }
};
}
