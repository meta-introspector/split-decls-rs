// Generated macro for impl_70 (impl)
macro_rules! Depcrate_addrimpl_70 {
() => {
// Module: crate::addr
// Provides: {"impl_70"}
// Dependencies: {}
impl Sealed for SocketAddrV6 { type Iter = std :: option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { Sealed :: to_socket_addrs (& SocketAddr :: V6 (* self)) } }
};
}
