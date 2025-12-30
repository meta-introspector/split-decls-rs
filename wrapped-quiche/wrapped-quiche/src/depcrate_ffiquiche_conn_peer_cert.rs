// Generated macro for quiche_conn_peer_cert (function)
macro_rules! Depcrate_ffiquiche_conn_peer_cert {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_peer_cert"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_peer_cert (conn : & Connection , out : & mut * const u8 , out_len : & mut size_t ,) { match conn . peer_cert () { Some (peer_cert) => { * out = peer_cert . as_ptr () ; * out_len = peer_cert . len () ; } , None => * out_len = 0 , } }
};
}
