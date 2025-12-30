// Generated macro for sockaddrBox (enum)
macro_rules! Depcrate_syscalls_socketsockaddrBox {
() => {
// Module: crate::syscalls::socket
// Provides: {"sockaddrBox"}
// Dependencies: {}
# [derive (Clone , Debug)] pub enum sockaddrBox { sockaddr (Box < sockaddr >) , sockaddr_in (Box < sockaddr_in >) , sockaddr_in6 (Box < sockaddr_in6 >) , sockaddr_un (Box < sockaddr_un >) , # [cfg (feature = "vsock")] sockaddr_vm (Box < sockaddr_vm >) , }
};
}
