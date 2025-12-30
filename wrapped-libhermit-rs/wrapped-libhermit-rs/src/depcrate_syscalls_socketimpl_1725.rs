// Generated macro for impl_1725 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1725 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1725"}
// Dependencies: {}
# [cfg (feature = "net")] impl From < sockaddr_in6 > for IpEndpoint { fn from (addr : sockaddr_in6) -> IpEndpoint { let port = u16 :: from_be (addr . sin6_port) ; let s6_addr = addr . sin6_addr . s6_addr ; let a0 = (u16 :: from (s6_addr [0]) << 8) | u16 :: from (s6_addr [1]) ; let a1 = (u16 :: from (s6_addr [2]) << 8) | u16 :: from (s6_addr [3]) ; let a2 = (u16 :: from (s6_addr [4]) << 8) | u16 :: from (s6_addr [5]) ; let a3 = (u16 :: from (s6_addr [6]) << 8) | u16 :: from (s6_addr [7]) ; let a4 = (u16 :: from (s6_addr [8]) << 8) | u16 :: from (s6_addr [9]) ; let a5 = (u16 :: from (s6_addr [10]) << 8) | u16 :: from (s6_addr [11]) ; let a6 = (u16 :: from (s6_addr [12]) << 8) | u16 :: from (s6_addr [13]) ; let a7 = (u16 :: from (s6_addr [14]) << 8) | u16 :: from (s6_addr [15]) ; let address = IpAddress :: v6 (a0 , a1 , a2 , a3 , a4 , a5 , a6 , a7) ; IpEndpoint :: from ((address , port)) } }
};
}
