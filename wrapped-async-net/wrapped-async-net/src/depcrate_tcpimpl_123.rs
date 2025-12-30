// Generated macro for impl_123 (impl)
macro_rules! Depcrate_tcpimpl_123 {
() => {
// Module: crate::tcp
// Provides: {"impl_123"}
// Dependencies: {}
impl From < TcpStream > for Arc < Async < std :: net :: TcpStream > > { fn from (val : TcpStream) -> Self { val . inner } }
};
}
