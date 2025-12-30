// Generated macro for impl_1717 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1717 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1717"}
// Dependencies: {}
# [cfg (feature = "vsock")] impl From < VsockEndpoint > for sockaddr_vm { fn from (endpoint : VsockEndpoint) -> Self { Self { svm_len : core :: mem :: size_of :: < sockaddr_vm > () . try_into () . unwrap () , svm_family : Af :: Vsock . into () , svm_port : endpoint . port , svm_cid : endpoint . cid , .. Default :: default () } } }
};
}
