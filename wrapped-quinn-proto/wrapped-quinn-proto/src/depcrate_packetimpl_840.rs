// Generated macro for impl_840 (impl)
macro_rules! Depcrate_packetimpl_840 {
() => {
// Module: crate::packet
// Provides: {"impl_840"}
// Dependencies: {}
impl LongHeaderType { fn from_byte (b : u8) -> Result < Self , PacketDecodeError > { use { LongHeaderType :: * , LongType :: * } ; debug_assert ! (b & LONG_HEADER_FORM != 0 , "not a long packet") ; Ok (match (b & 0x30) >> 4 { 0x0 => Initial , 0x1 => Standard (ZeroRtt) , 0x2 => Standard (Handshake) , 0x3 => Retry , _ => unreachable ! () , }) } }
};
}
