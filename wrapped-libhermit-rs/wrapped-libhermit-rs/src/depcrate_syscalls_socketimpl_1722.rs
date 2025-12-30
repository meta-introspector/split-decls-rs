// Generated macro for impl_1722 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1722 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1722"}
// Dependencies: {}
impl From < SocketAddrV4 > for sockaddr_in { fn from (value : SocketAddrV4) -> Self { Self { sin_len : mem :: size_of :: < Self > () . try_into () . unwrap () , sin_family : Af :: Inet . into () , sin_port : value . port () . to_be () , sin_addr : (* value . ip ()) . into () , sin_zero : Default :: default () , } } }
};
}
