// Generated macro for quiche_connection_id_iter_next (function)
macro_rules! Depcrate_ffiquiche_connection_id_iter_next {
() => {
// Module: crate::ffi
// Provides: {"quiche_connection_id_iter_next"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_connection_id_iter_next (iter : & mut ConnectionIdIter , out : & mut * const u8 , out_len : & mut size_t ,) -> bool { if let Some (conn_id) = iter . next () { let id = conn_id . as_ref () ; * out = id . as_ptr () ; * out_len = id . len () ; return true ; } false }
};
}
