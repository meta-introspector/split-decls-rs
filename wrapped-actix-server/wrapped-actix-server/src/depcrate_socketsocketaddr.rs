// Generated macro for SocketAddr (enum)
macro_rules! Depcrate_socketSocketAddr {
() => {
// Module: crate::socket
// Provides: {"SocketAddr"}
// Dependencies: {}
pub (crate) enum SocketAddr { Unknown , Tcp (StdSocketAddr) , # [cfg (unix)] Uds (std :: os :: unix :: net :: SocketAddr) , }
};
}
