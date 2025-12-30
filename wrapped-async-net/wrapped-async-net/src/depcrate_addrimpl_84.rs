// Generated macro for impl_84 (impl)
macro_rules! Depcrate_addrimpl_84 {
() => {
// Module: crate::addr
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a > Sealed for & 'a [SocketAddr] { type Iter = std :: iter :: Cloned < std :: slice :: Iter < 'a , SocketAddr > > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { ToSocketAddrsFuture :: Ready (Ok (self . iter () . cloned ())) } }
};
}
