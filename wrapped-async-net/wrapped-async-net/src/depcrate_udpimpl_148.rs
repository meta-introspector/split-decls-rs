// Generated macro for impl_148 (impl)
macro_rules! Depcrate_udpimpl_148 {
() => {
// Module: crate::udp
// Provides: {"impl_148"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < OwnedFd > for UdpSocket { type Error = io :: Error ; fn try_from (value : OwnedFd) -> Result < Self , Self :: Error > { Self :: try_from (std :: net :: UdpSocket :: from (value)) } }
};
}
