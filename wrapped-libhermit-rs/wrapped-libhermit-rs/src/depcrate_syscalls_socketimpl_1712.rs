// Generated macro for impl_1712 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1712 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1712"}
// Dependencies: {}
impl From < SocketAddr > for sockaddrBox { fn from (value : SocketAddr) -> Self { match value { SocketAddr :: V4 (socket_addr_v4) => Self :: sockaddr_in (Box :: new (socket_addr_v4 . into ())) , SocketAddr :: V6 (socket_addr_v6) => Self :: sockaddr_in6 (Box :: new (socket_addr_v6 . into ())) , } } }
};
}
