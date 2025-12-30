// Generated macro for impl_1715 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1715 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1715"}
// Dependencies: {}
# [cfg (feature = "vsock")] impl From < sockaddr_vm > for VsockListenEndpoint { fn from (addr : sockaddr_vm) -> VsockListenEndpoint { let port = addr . svm_port ; let cid = if addr . svm_cid < u32 :: MAX { Some (addr . svm_cid) } else { None } ; VsockListenEndpoint :: new (port , cid) } }
};
}
