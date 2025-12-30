// Generated macro for close_conn_critical_stream (function)
macro_rules! Depcrate_h3close_conn_critical_stream {
() => {
// Module: crate::h3
// Provides: {"close_conn_critical_stream"}
// Dependencies: {}
fn close_conn_critical_stream < F : BufFactory > (conn : & mut super :: Connection < F > ,) -> Result < () > { conn . close (true , Error :: ClosedCriticalStream . to_wire () , b"Critical stream closed." ,) ? ; Err (Error :: ClosedCriticalStream) }
};
}
