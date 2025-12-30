// Generated macro for quiche_conn_migrate (function)
macro_rules! Depcrate_ffiquiche_conn_migrate {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_migrate"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_migrate (conn : & mut Connection , local : & sockaddr , local_len : socklen_t , peer : & sockaddr , peer_len : socklen_t , seq : * mut u64 ,) -> c_int { let local = std_addr_from_c (local , local_len) ; let peer = std_addr_from_c (peer , peer_len) ; match conn . migrate (local , peer) { Ok (v) => { unsafe { * seq = v } 0 } , Err (e) => e . to_c () as c_int , } }
};
}
