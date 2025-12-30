// Generated macro for impl_96 (impl)
macro_rules! Depcrate_netimpl_96 {
() => {
// Module: crate::net
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a > AcceptAddrs < 'a > { # [doc = " Returns the local socket address contained in this buffer."] pub fn local (& self) -> Option < SocketAddr > { unsafe { ptrs_to_socket_addr (self . local , self . local_len) } } # [doc = " Returns the remote socket address contained in this buffer."] pub fn remote (& self) -> Option < SocketAddr > { unsafe { ptrs_to_socket_addr (self . remote , self . remote_len) } } }
};
}
