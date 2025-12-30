// Generated macro for impl_837 (impl)
macro_rules! Depcrate_packetimpl_837 {
() => {
// Module: crate::packet
// Provides: {"impl_837"}
// Dependencies: {}
impl ConnectionIdParser for FixedLengthConnectionIdParser { fn parse (& self , buffer : & mut dyn Buf) -> Result < ConnectionId , PacketDecodeError > { (buffer . remaining () >= self . expected_len) . then (| | ConnectionId :: from_buf (buffer , self . expected_len)) . ok_or (PacketDecodeError :: InvalidHeader ("packet too small")) } }
};
}
