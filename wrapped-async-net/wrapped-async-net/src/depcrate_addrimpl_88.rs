// Generated macro for impl_88 (impl)
macro_rules! Depcrate_addrimpl_88 {
() => {
// Module: crate::addr
// Provides: {"impl_88"}
// Dependencies: {}
impl Sealed for String { type Iter = std :: vec :: IntoIter < SocketAddr > ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { Sealed :: to_socket_addrs (& * * self) } }
};
}
