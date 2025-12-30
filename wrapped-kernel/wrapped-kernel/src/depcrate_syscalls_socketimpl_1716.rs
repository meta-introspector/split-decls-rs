// Generated macro for impl_1716 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1716 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1716"}
// Dependencies: {}
# [cfg (feature = "vsock")] impl From < sockaddr_vm > for VsockEndpoint { fn from (addr : sockaddr_vm) -> VsockEndpoint { let port = addr . svm_port ; let cid = addr . svm_cid ; VsockEndpoint :: new (port , cid) } }
};
}
