// Generated macro for impl_72 (impl)
macro_rules! Depcrate_addrimpl_72 {
() => {
// Module: crate::addr
// Provides: {"impl_72"}
// Dependencies: {}
impl Sealed for (IpAddr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { let (ip , port) = * self ; match ip { IpAddr :: V4 (a) => Sealed :: to_socket_addrs (& (a , port)) , IpAddr :: V6 (a) => Sealed :: to_socket_addrs (& (a , port)) , } } }
};
}
