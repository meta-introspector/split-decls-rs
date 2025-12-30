// Generated macro for quiche_h3_dgram_enabled_by_peer (function)
macro_rules! Depcrate_h3_ffiquiche_h3_dgram_enabled_by_peer {
() => {
// Module: crate::h3::ffi
// Provides: {"quiche_h3_dgram_enabled_by_peer"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_h3_dgram_enabled_by_peer (conn : & h3 :: Connection , quic_conn : & Connection ,) -> bool { conn . dgram_enabled_by_peer (quic_conn) }
};
}
