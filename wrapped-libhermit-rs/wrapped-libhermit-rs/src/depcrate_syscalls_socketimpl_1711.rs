// Generated macro for impl_1711 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1711 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1711"}
// Dependencies: {}
impl sockaddrBox { pub fn into_raw (self) -> * mut sockaddr { match self { sockaddrBox :: sockaddr (sockaddr) => Box :: into_raw (sockaddr) , sockaddrBox :: sockaddr_in (sockaddr_in) => Box :: into_raw (sockaddr_in) . cast () , sockaddrBox :: sockaddr_in6 (sockaddr_in6) => Box :: into_raw (sockaddr_in6) . cast () , sockaddrBox :: sockaddr_un (sockaddr_un) => Box :: into_raw (sockaddr_un) . cast () , # [cfg (feature = "vsock")] sockaddrBox :: sockaddr_vm (sockaddr_vm) => Box :: into_raw (sockaddr_vm) . cast () , } } pub fn as_ref (& self) -> sockaddrRef < '_ > { match self { Self :: sockaddr (sockaddr) => sockaddrRef :: sockaddr (sockaddr . as_ref ()) , Self :: sockaddr_in (sockaddr_in) => sockaddrRef :: sockaddr_in (sockaddr_in . as_ref ()) , Self :: sockaddr_in6 (sockaddr_in6) => sockaddrRef :: sockaddr_in6 (sockaddr_in6 . as_ref ()) , Self :: sockaddr_un (sockaddr_un) => sockaddrRef :: sockaddr_un (sockaddr_un . as_ref ()) , # [cfg (feature = "vsock")] Self :: sockaddr_vm (sockaddr_vm) => sockaddrRef :: sockaddr_vm (sockaddr_vm . as_ref ()) , } } }
};
}
