// Generated macro for impl_86 (impl)
macro_rules! Depcrate_addrimpl_86 {
() => {
// Module: crate::addr
// Provides: {"impl_86"}
// Dependencies: {}
impl < T : Sealed + ? Sized > Sealed for & T { type Iter = T :: Iter ; fn to_socket_addrs (& self) -> ToSocketAddrsFuture < Self :: Iter > { Sealed :: to_socket_addrs (& * * self) } }
};
}
