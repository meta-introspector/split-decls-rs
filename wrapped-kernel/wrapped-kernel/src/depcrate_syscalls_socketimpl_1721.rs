// Generated macro for impl_1721 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1721 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1721"}
// Dependencies: {}
# [cfg (feature = "net")] impl From < IpEndpoint > for sockaddr_in { fn from (endpoint : IpEndpoint) -> Self { match endpoint . addr { IpAddress :: Ipv4 (ip) => { let sin_addr = in_addr { s_addr : u32 :: from_ne_bytes (ip . octets ()) , } ; Self { sin_len : core :: mem :: size_of :: < sockaddr_in > () . try_into () . unwrap () , sin_port : endpoint . port . to_be () , sin_family : Af :: Inet . into () , sin_addr , .. Default :: default () } } IpAddress :: Ipv6 (_) => panic ! ("Unable to convert IPv6 address to sockadd_in") , } } }
};
}
