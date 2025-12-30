// Generated macro for impl_74 (impl)
macro_rules! Depcrate_addrimpl_74 {
() => {
// Module: crate::addr
// Provides: {"impl_74"}
// Dependencies: {}
impl Sealed for (Ipv4Addr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { let (ip , port) = * self ; Sealed :: to_socket_addrs (& SocketAddrV4 :: new (ip , port)) } }
};
}
