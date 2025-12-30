// Generated macro for impl_1727 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1727 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1727"}
// Dependencies: {}
impl From < SocketAddrV6 > for sockaddr_in6 { fn from (value : SocketAddrV6) -> Self { Self { sin6_len : mem :: size_of :: < Self > () . try_into () . unwrap () , sin6_family : Af :: Inet6 . into () , sin6_port : value . port () . to_be () , sin6_flowinfo : Default :: default () , sin6_addr : (* value . ip ()) . into () , sin6_scope_id : Default :: default () , } } }
};
}
