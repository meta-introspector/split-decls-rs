// Generated macro for impl_127 (impl)
macro_rules! Depcrate_tcpimpl_127 {
() => {
// Module: crate::tcp
// Provides: {"impl_127"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < OwnedFd > for TcpStream { type Error = io :: Error ; fn try_from (value : OwnedFd) -> Result < Self , Self :: Error > { Self :: try_from (std :: net :: TcpStream :: from (value)) } }
};
}
