// Generated macro for quiche_socket_addr_iter_next (function)
macro_rules! Depcrate_ffiquiche_socket_addr_iter_next {
() => {
// Module: crate::ffi
// Provides: {"quiche_socket_addr_iter_next"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_socket_addr_iter_next (iter : & mut SocketAddrIter , peer : & mut sockaddr_storage , peer_len : * mut socklen_t ,) -> bool { if let Some (v) = iter . next () { unsafe { * peer_len = std_addr_to_c (& v , peer) } return true ; } false }
};
}
