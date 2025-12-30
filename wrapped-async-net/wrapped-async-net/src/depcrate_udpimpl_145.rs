// Generated macro for impl_145 (impl)
macro_rules! Depcrate_udpimpl_145 {
() => {
// Module: crate::udp
// Provides: {"impl_145"}
// Dependencies: {}
impl From < UdpSocket > for Arc < Async < std :: net :: UdpSocket > > { fn from (val : UdpSocket) -> Self { val . inner } }
};
}
