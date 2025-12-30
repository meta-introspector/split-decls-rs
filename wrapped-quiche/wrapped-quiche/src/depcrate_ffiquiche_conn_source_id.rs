// Generated macro for quiche_conn_source_id (function)
macro_rules! Depcrate_ffiquiche_conn_source_id {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_source_id"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_source_id (conn : & Connection , out : & mut * const u8 , out_len : & mut size_t ,) { let conn_id = conn . source_id () ; let id = conn_id . as_ref () ; * out = id . as_ptr () ; * out_len = id . len () ; }
};
}
