// Generated macro for quiche_conn_dgram_purge_outgoing (function)
macro_rules! Depcrate_ffiquiche_conn_dgram_purge_outgoing {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_dgram_purge_outgoing"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_dgram_purge_outgoing (conn : & mut Connection , f : extern "C" fn (* const u8 , size_t) -> bool ,) { conn . dgram_purge_outgoing (| d : & [u8] | -> bool { let ptr : * const u8 = d . as_ptr () ; let len : size_t = d . len () ; f (ptr , len) }) ; }
};
}
