// Generated macro for impl_82 (impl)
macro_rules! Depcrate_addrimpl_82 {
() => {
// Module: crate::addr
// Provides: {"impl_82"}
// Dependencies: {}
impl Sealed for str { type Iter = std :: vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { if let Ok (addr) = self . parse () { return ToSocketAddrsFuture :: Ready (Ok (vec ! [addr] . into_iter ())) ; } let addr = self . to_string () ; let future = unblock (move | | std :: net :: ToSocketAddrs :: to_socket_addrs (addr . as_str ())) ; ToSocketAddrsFuture :: Resolving (Box :: pin (future)) } }
};
}
