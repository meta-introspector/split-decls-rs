// Generated macro for quiche_conn_session (function)
macro_rules! Depcrate_ffiquiche_conn_session {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_session"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_session (conn : & Connection , out : & mut * const u8 , out_len : & mut size_t ,) { match conn . session () { Some (session) => { * out = session . as_ptr () ; * out_len = session . len () ; } , None => * out_len = 0 , } }
};
}
