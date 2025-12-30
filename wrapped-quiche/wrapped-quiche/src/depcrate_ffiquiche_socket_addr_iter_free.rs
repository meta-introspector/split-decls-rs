// Generated macro for quiche_socket_addr_iter_free (function)
macro_rules! Depcrate_ffiquiche_socket_addr_iter_free {
() => {
// Module: crate::ffi
// Provides: {"quiche_socket_addr_iter_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn quiche_socket_addr_iter_free (iter : * mut SocketAddrIter) { drop (unsafe { Box :: from_raw (iter) }) ; }
};
}
