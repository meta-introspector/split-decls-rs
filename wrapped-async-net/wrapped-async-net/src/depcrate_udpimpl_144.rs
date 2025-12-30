// Generated macro for impl_144 (impl)
macro_rules! Depcrate_udpimpl_144 {
() => {
// Module: crate::udp
// Provides: {"impl_144"}
// Dependencies: {}
impl TryFrom < std :: net :: UdpSocket > for UdpSocket { type Error = io :: Error ; fn try_from (socket : std :: net :: UdpSocket) -> io :: Result < UdpSocket > { Ok (UdpSocket :: new (Arc :: new (Async :: new (socket) ?))) } }
};
}
