// Generated macro for impl_1713 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1713 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1713"}
// Dependencies: {}
impl sockaddrRef < '_ > { pub fn addrlen (self) -> u8 { match self { sockaddrRef :: sockaddr (sockaddr) => sockaddr . sa_len , sockaddrRef :: sockaddr_in (sockaddr_in) => sockaddr_in . sin_len , sockaddrRef :: sockaddr_in6 (sockaddr_in6) => sockaddr_in6 . sin6_len , sockaddrRef :: sockaddr_un (sockaddr_un) => sockaddr_un . sun_len , # [cfg (feature = "vsock")] sockaddrRef :: sockaddr_vm (sockaddr_vm) => sockaddr_vm . svm_len , } } }
};
}
