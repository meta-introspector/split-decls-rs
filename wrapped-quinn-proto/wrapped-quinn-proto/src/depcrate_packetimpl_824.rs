// Generated macro for impl_824 (impl)
macro_rules! Depcrate_packetimpl_824 {
() => {
// Module: crate::packet
// Provides: {"impl_824"}
// Dependencies: {}
impl From < InitialPacket > for Packet { fn from (x : InitialPacket) -> Self { Self { header : Header :: Initial (x . header) , header_data : x . header_data , payload : x . payload , } } }
};
}
