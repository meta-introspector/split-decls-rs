// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
impl TryFrom < std :: net :: TcpStream > for Async < std :: net :: TcpStream > { type Error = io :: Error ; fn try_from (stream : std :: net :: TcpStream) -> io :: Result < Self > { Async :: new (stream) } }
};
}
