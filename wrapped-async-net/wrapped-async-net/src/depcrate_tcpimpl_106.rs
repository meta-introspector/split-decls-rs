// Generated macro for impl_106 (impl)
macro_rules! Depcrate_tcpimpl_106 {
() => {
// Module: crate::tcp
// Provides: {"impl_106"}
// Dependencies: {}
impl From < TcpListener > for Arc < Async < std :: net :: TcpListener > > { fn from (val : TcpListener) -> Self { val . inner } }
};
}
