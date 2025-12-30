// Generated macro for close_conn_if_critical_stream_finished (function)
macro_rules! Depcrate_h3close_conn_if_critical_stream_finished {
() => {
// Module: crate::h3
// Provides: {"close_conn_if_critical_stream_finished"}
// Dependencies: {}
fn close_conn_if_critical_stream_finished < F : BufFactory > (conn : & mut super :: Connection < F > , stream_id : u64 ,) -> Result < () > { if conn . stream_finished (stream_id) { close_conn_critical_stream (conn) ? ; } Ok (()) }
};
}
