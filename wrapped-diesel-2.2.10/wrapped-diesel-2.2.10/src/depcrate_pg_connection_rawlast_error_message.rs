// Generated macro for last_error_message (function)
macro_rules! Depcrate_pg_connection_rawlast_error_message {
() => {
// Module: crate::pg::connection::raw
// Provides: {"last_error_message"}
// Dependencies: {}
fn last_error_message (conn : * const PGconn) -> String { unsafe { let error_ptr = PQerrorMessage (conn) ; let bytes = CStr :: from_ptr (error_ptr) . to_bytes () ; String :: from_utf8_lossy (bytes) . to_string () } }
};
}
