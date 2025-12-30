// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__std_nettest {
() => {
// Module: crate::arbitrary::_std::net
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { no_panic_test ! (addr_parse_error => AddrParseError , ipv4_addr => Ipv4Addr , ipv6_addr => Ipv6Addr , socket_addr_v4 => SocketAddrV4 , socket_addr_v6 => SocketAddrV6 , ip_addr => IpAddr , shutdown => Shutdown , socket_addr => SocketAddr) ; # [cfg (feature = "unstable")] no_panic_test ! (ipv6_multicast_scope => Ipv6MulticastScope) ; }
};
}
