// Generated macro for impl_143 (impl)
macro_rules! Depcrate_udpimpl_143 {
() => {
// Module: crate::udp
// Provides: {"impl_143"}
// Dependencies: {}
impl From < Async < std :: net :: UdpSocket > > for UdpSocket { fn from (socket : Async < std :: net :: UdpSocket >) -> UdpSocket { UdpSocket :: new (Arc :: new (socket)) } }
};
}
