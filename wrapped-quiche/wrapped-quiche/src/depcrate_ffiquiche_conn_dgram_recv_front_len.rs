// Generated macro for quiche_conn_dgram_recv_front_len (function)
macro_rules! Depcrate_ffiquiche_conn_dgram_recv_front_len {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_dgram_recv_front_len"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_dgram_recv_front_len (conn : & Connection) -> ssize_t { match conn . dgram_recv_front_len () { None => Error :: Done . to_c () , Some (v) => v as ssize_t , } }
};
}
