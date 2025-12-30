// Generated macro for impl_151 (impl)
macro_rules! Depcrate_udpimpl_151 {
() => {
// Module: crate::udp
// Provides: {"impl_151"}
// Dependencies: {}
# [cfg (windows)] impl TryFrom < OwnedSocket > for UdpSocket { type Error = io :: Error ; fn try_from (value : OwnedSocket) -> Result < Self , Self :: Error > { Self :: try_from (std :: net :: UdpSocket :: from (value)) } }
};
}
