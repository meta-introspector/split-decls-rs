// Generated macro for impl_124 (impl)
macro_rules! Depcrate_tcpimpl_124 {
() => {
// Module: crate::tcp
// Provides: {"impl_124"}
// Dependencies: {}
impl TryFrom < std :: net :: TcpStream > for TcpStream { type Error = io :: Error ; fn try_from (stream : std :: net :: TcpStream) -> io :: Result < TcpStream > { Ok (TcpStream :: new (Arc :: new (Async :: new (stream) ?))) } }
};
}
