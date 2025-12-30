// Generated macro for quiche_conn_server_name (function)
macro_rules! Depcrate_ffiquiche_conn_server_name {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_server_name"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_server_name (conn : & Connection , out : & mut * const u8 , out_len : & mut size_t ,) { match conn . server_name () { Some (server_name) => { * out = server_name . as_ptr () ; * out_len = server_name . len () ; } , None => * out_len = 0 , } }
};
}
