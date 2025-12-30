// Generated macro for impl_1720 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1720 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1720"}
// Dependencies: {}
# [cfg (feature = "net")] impl From < sockaddr_in > for IpEndpoint { fn from (addr : sockaddr_in) -> IpEndpoint { let port = u16 :: from_be (addr . sin_port) ; let s_addr = addr . sin_addr . s_addr . to_ne_bytes () ; let address = IpAddress :: v4 (s_addr [0] , s_addr [1] , s_addr [2] , s_addr [3]) ; IpEndpoint :: from ((address , port)) } }
};
}
