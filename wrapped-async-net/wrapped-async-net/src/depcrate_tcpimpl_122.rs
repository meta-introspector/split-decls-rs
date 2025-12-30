// Generated macro for impl_122 (impl)
macro_rules! Depcrate_tcpimpl_122 {
() => {
// Module: crate::tcp
// Provides: {"impl_122"}
// Dependencies: {}
impl From < Async < std :: net :: TcpStream > > for TcpStream { fn from (stream : Async < std :: net :: TcpStream >) -> TcpStream { TcpStream :: new (Arc :: new (stream)) } }
};
}
