// Generated macro for sockaddrRef (enum)
macro_rules! Depcrate_syscalls_socketsockaddrRef {
() => {
// Module: crate::syscalls::socket
// Provides: {"sockaddrRef"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] pub enum sockaddrRef < 'a > { sockaddr (& 'a sockaddr) , sockaddr_in (& 'a sockaddr_in) , sockaddr_in6 (& 'a sockaddr_in6) , sockaddr_un (& 'a sockaddr_un) , # [cfg (feature = "vsock")] sockaddr_vm (& 'a sockaddr_vm) , }
};
}
