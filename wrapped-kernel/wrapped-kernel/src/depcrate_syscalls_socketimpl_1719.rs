// Generated macro for impl_1719 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1719 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1719"}
// Dependencies: {}
# [cfg (feature = "net")] impl From < sockaddr_in > for IpListenEndpoint { fn from (addr : sockaddr_in) -> IpListenEndpoint { let port = u16 :: from_be (addr . sin_port) ; if addr . sin_addr . s_addr == 0 { IpListenEndpoint { addr : None , port } } else { let s_addr = addr . sin_addr . s_addr . to_ne_bytes () ; let address = IpAddress :: v4 (s_addr [0] , s_addr [1] , s_addr [2] , s_addr [3]) ; IpListenEndpoint :: from ((address , port)) } } }
};
}
