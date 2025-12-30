// Generated macro for PacketHeader (struct)
macro_rules! Depcrate_events_quicPacketHeader {
() => {
// Module: crate::events::quic
// Provides: {"PacketHeader"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Clone , Serialize , Deserialize , PartialEq , Eq , Debug , Default)] pub struct PacketHeader { pub packet_type : PacketType , pub packet_number : Option < u64 > , pub flags : Option < u8 > , pub token : Option < Token > , pub length : Option < u16 > , pub version : Option < Bytes > , pub scil : Option < u8 > , pub dcil : Option < u8 > , pub scid : Option < Bytes > , pub dcid : Option < Bytes > , }
};
}
