// Generated macro for quiche_conn_migrate_source (function)
macro_rules! Depcrate_ffiquiche_conn_migrate_source {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_migrate_source"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_migrate_source (conn : & mut Connection , local : & sockaddr , local_len : socklen_t , seq : * mut u64 ,) -> c_int { let local = std_addr_from_c (local , local_len) ; match conn . migrate_source (local) { Ok (v) => { unsafe { * seq = v } 0 } , Err (e) => e . to_c () as c_int , } }
};
}
