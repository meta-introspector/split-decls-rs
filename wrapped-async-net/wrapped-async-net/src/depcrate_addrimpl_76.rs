// Generated macro for impl_76 (impl)
macro_rules! Depcrate_addrimpl_76 {
() => {
// Module: crate::addr
// Provides: {"impl_76"}
// Dependencies: {}
impl Sealed for (Ipv6Addr , u16) { type Iter = std :: option :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { let (ip , port) = * self ; Sealed :: to_socket_addrs (& SocketAddrV6 :: new (ip , port , 0 , 0)) } }
};
}
