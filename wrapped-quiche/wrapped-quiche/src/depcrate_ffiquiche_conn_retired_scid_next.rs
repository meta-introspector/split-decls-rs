// Generated macro for quiche_conn_retired_scid_next (function)
macro_rules! Depcrate_ffiquiche_conn_retired_scid_next {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_retired_scid_next"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_retired_scid_next (conn : & mut Connection , out : & mut * const u8 , out_len : & mut size_t ,) -> bool { match conn . retired_scid_next () { None => false , Some (conn_id) => { let id = conn_id . as_ref () ; * out = id . as_ptr () ; * out_len = id . len () ; true } , } }
};
}
