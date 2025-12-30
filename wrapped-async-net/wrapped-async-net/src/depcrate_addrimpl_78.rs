// Generated macro for impl_78 (impl)
macro_rules! Depcrate_addrimpl_78 {
() => {
// Module: crate::addr
// Provides: {"impl_78"}
// Dependencies: {}
impl Sealed for (& str , u16) { type Iter = std :: vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { let (host , port) = * self ; if let Ok (addr) = host . parse :: < Ipv4Addr > () { let addr = SocketAddrV4 :: new (addr , port) ; return ToSocketAddrsFuture :: Ready (Ok (vec ! [SocketAddr :: V4 (addr)] . into_iter ())) ; } if let Ok (addr) = host . parse :: < Ipv6Addr > () { let addr = SocketAddrV6 :: new (addr , port , 0 , 0) ; return ToSocketAddrsFuture :: Ready (Ok (vec ! [SocketAddr :: V6 (addr)] . into_iter ())) ; } let host = host . to_string () ; let future = unblock (move | | { let addr = (host . as_str () , port) ; ToSocketAddrs :: to_socket_addrs (& addr) }) ; ToSocketAddrsFuture :: Resolving (Box :: pin (future)) } }
};
}
