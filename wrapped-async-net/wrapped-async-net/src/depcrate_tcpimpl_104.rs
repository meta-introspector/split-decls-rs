// Generated macro for impl_104 (impl)
macro_rules! Depcrate_tcpimpl_104 {
() => {
// Module: crate::tcp
// Provides: {"impl_104"}
// Dependencies: {}
impl From < Async < std :: net :: TcpListener > > for TcpListener { fn from (listener : Async < std :: net :: TcpListener >) -> TcpListener { TcpListener :: new (Arc :: new (listener)) } }
};
}
