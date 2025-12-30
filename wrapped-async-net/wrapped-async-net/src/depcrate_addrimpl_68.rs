// Generated macro for impl_68 (impl)
macro_rules! Depcrate_addrimpl_68 {
() => {
// Module: crate::addr
// Provides: {"impl_68"}
// Dependencies: {}
impl Sealed for SocketAddrV4 { type Iter = std :: option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { Sealed :: to_socket_addrs (& SocketAddr :: V4 (* self)) } }
};
}
