// Generated macro for impl_130 (impl)
macro_rules! Depcrate_tcpimpl_130 {
() => {
// Module: crate::tcp
// Provides: {"impl_130"}
// Dependencies: {}
# [cfg (windows)] impl TryFrom < OwnedSocket > for TcpStream { type Error = io :: Error ; fn try_from (value : OwnedSocket) -> Result < Self , Self :: Error > { Self :: try_from (std :: net :: TcpStream :: from (value)) } }
};
}
