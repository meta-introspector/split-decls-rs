// Generated macro for impl_109 (impl)
macro_rules! Depcrate_tcpimpl_109 {
() => {
// Module: crate::tcp
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < OwnedFd > for TcpListener { type Error = io :: Error ; fn try_from (value : OwnedFd) -> Result < Self , Self :: Error > { Self :: try_from (std :: net :: TcpListener :: from (value)) } }
};
}
