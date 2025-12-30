// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
impl TryFrom < std :: net :: UdpSocket > for Async < std :: net :: UdpSocket > { type Error = io :: Error ; fn try_from (socket : std :: net :: UdpSocket) -> io :: Result < Self > { Async :: new (socket) } }
};
}
