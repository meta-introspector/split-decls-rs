// Generated macro for impl_112 (impl)
macro_rules! Depcrate_tcpimpl_112 {
() => {
// Module: crate::tcp
// Provides: {"impl_112"}
// Dependencies: {}
# [cfg (windows)] impl TryFrom < OwnedSocket > for TcpListener { type Error = io :: Error ; fn try_from (value : OwnedSocket) -> Result < Self , Self :: Error > { Self :: try_from (std :: net :: TcpListener :: from (value)) } }
};
}
