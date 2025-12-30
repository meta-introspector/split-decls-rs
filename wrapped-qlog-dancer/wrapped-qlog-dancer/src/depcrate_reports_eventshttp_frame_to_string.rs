// Generated macro for http_frame_to_string (function)
macro_rules! Depcrate_reports_eventshttp_frame_to_string {
() => {
// Module: crate::reports::events
// Provides: {"http_frame_to_string"}
// Dependencies: {}
fn http_frame_to_string (frame : & Http3Frame) -> String { let mut s = String :: new () ; match frame { Http3Frame :: Data { raw } => { s += " DATA" ; if let Some (r) = raw { printyo ! ("len" , r . length , s) ; } } , Http3Frame :: Headers { headers } => { s += " HEADERS {" ; for header in headers { s += & format ! ("{}: {}, " , header . name , header . value) ; } s += "}" ; } , Http3Frame :: CancelPush { push_id } => { s += & format ! (" CANCEL_PUSH {{id={push_id}}}") ; } , Http3Frame :: Settings { .. } => { s += " SETTINGS {{todo}}" ; } Http3Frame :: PushPromise { .. } => { s += " PUSH_PROMISE {{todo}}" ; } Http3Frame :: Goaway { id } => { s += & format ! (" GOAWAY {{id={id}}}") ; } Http3Frame :: MaxPushId { push_id } => { s += & format ! (" MAX_PUSH_ID {{id={push_id}}}") ; } Http3Frame :: PriorityUpdate { .. } => { s += " PRIORITY_UPDATE {{todo}}" ; } , Http3Frame :: Reserved { .. } => { s += " GREASE {{todo}}" ; } , Http3Frame :: Unknown { frame_type_value , .. } => { s += & format ! (" UNKNOWN {{ty={frame_type_value}}}") ; } } s }
};
}
