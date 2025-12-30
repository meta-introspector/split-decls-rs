// Generated macro for impl_80 (impl)
macro_rules! Depcrate_addrimpl_80 {
() => {
// Module: crate::addr
// Provides: {"impl_80"}
// Dependencies: {}
impl Sealed for (String , u16) { type Iter = std :: vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { Sealed :: to_socket_addrs (& (& * self . 0 , self . 1)) } }
};
}
