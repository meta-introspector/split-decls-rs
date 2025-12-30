// Generated macro for impl_66 (impl)
macro_rules! Depcrate_addrimpl_66 {
() => {
// Module: crate::addr
// Provides: {"impl_66"}
// Dependencies: {}
impl Sealed for SocketAddr { type Iter = std :: option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { ToSocketAddrsFuture :: Ready (Ok (Some (* self) . into_iter ())) } }
};
}
