// Generated macro for impl_105 (impl)
macro_rules! Depcrate_tcpimpl_105 {
() => {
// Module: crate::tcp
// Provides: {"impl_105"}
// Dependencies: {}
impl TryFrom < std :: net :: TcpListener > for TcpListener { type Error = io :: Error ; fn try_from (listener : std :: net :: TcpListener) -> io :: Result < TcpListener > { Ok (TcpListener :: new (Arc :: new (Async :: new (listener) ?))) } }
};
}
