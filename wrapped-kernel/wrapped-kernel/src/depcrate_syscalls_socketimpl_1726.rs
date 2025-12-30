// Generated macro for impl_1726 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1726 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1726"}
// Dependencies: {}
# [cfg (feature = "net")] impl From < IpEndpoint > for sockaddr_in6 { fn from (endpoint : IpEndpoint) -> Self { match endpoint . addr { IpAddress :: Ipv6 (ip) => { let mut in6_addr = in6_addr :: default () ; in6_addr . s6_addr . copy_from_slice (& ip . octets ()) ; Self { sin6_len : core :: mem :: size_of :: < sockaddr_in6 > () . try_into () . unwrap () , sin6_port : endpoint . port . to_be () , sin6_family : Af :: Inet6 . into () , sin6_addr : in6_addr , .. Default :: default () } } IpAddress :: Ipv4 (_) => panic ! ("Unable to convert IPv4 address to sockadd_in6") , } } }
};
}
