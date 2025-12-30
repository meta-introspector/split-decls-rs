// Generated macro for perform_thread_unsafe_library_initialization (function)
macro_rules! Depcrate_mysql_connection_rawperform_thread_unsafe_library_initialization {
() => {
// Module: crate::mysql::connection::raw
// Provides: {"perform_thread_unsafe_library_initialization"}
// Dependencies: {}
fn perform_thread_unsafe_library_initialization () { MYSQL_THREAD_UNSAFE_INIT . call_once (| | { let error_code = unsafe { ffi :: mysql_server_init (0 , ptr :: null_mut () , ptr :: null_mut ()) } ; if error_code != 0 { panic ! ("Unable to perform MySQL global initialization") ; } }) }
};
}
