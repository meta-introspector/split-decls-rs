// Generated macro for quiche_conn_paths_iter (function)
macro_rules! Depcrate_ffiquiche_conn_paths_iter {
() => {
// Module: crate::ffi
// Provides: {"quiche_conn_paths_iter"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_conn_paths_iter (conn : & Connection , from : & sockaddr , from_len : socklen_t ,) -> * mut SocketAddrIter { let addr = std_addr_from_c (from , from_len) ; Box :: into_raw (Box :: new (conn . paths_iter (addr))) }
};
}
