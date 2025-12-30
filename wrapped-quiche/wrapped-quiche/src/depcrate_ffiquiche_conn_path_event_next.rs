// Generated macro for quiche_conn_path_event_next (function)
macro_rules! Depcrate_ffiquiche_conn_path_event_next {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_path_event_next"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_path_event_next (conn : & mut Connection ,) -> * mut PathEvent { match conn . path_event_next () { Some (v) => Box :: into_raw (Box :: new (v)) , None => ptr :: null_mut () , } }
};
}
