// Generated macro for impl_134 (impl)
macro_rules! Depcrateimpl_134 {
() => {
// Module: crate
// Provides: {"impl_134"}
// Dependencies: {}
impl TryFrom < std :: net :: TcpListener > for Async < std :: net :: TcpListener > { type Error = io :: Error ; fn try_from (listener : std :: net :: TcpListener) -> io :: Result < Self > { Async :: new (listener) } }
};
}
