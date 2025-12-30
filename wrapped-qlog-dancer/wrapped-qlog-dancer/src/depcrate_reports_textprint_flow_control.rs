// Generated macro for print_flow_control (function)
macro_rules! Depcrate_reports_textprint_flow_control {
() => {
// Module: crate::reports::text
// Provides: {"print_flow_control"}
// Dependencies: {}
pub fn print_flow_control (data : & [LogFileData]) { println ! ("================") ; println ! ("flow control stuff") ; println ! ("================") ; for lf in data { println ! ("Session={}, host={}" , lf . datastore . session_id . unwrap_or (- 1) , lf . datastore . host . clone () . unwrap_or ("ERROR UNKNOWN" . to_string ())) ; println ! ("  Initial Client connection window, Initial Client Bidi Local Stream Window") ; println ! ("  {},{}" , lf . datastore . client_quic_tps . initial_max_data . unwrap_or (0) , lf . datastore . client_quic_tps . initial_max_stream_data_bidi_local . unwrap_or (0)) ; for (stream_id , points) in & lf . datastore . netlog_quic_client_side_window_updates { println ! ("  Stream {} flow control updates" , stream_id) ; println ! ("    Time, Value") ; for (time , val) in points { println ! ("    {},{}" , time , val) ; } } } }
};
}
