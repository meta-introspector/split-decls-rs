// Generated macro for PacketBuffered (struct)
macro_rules! Depcrate_events_quicPacketBuffered {
() => {
// Module: crate::events::quic
// Provides: {"PacketBuffered"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct PacketBuffered { pub header : Option < PacketHeader > , pub raw : Option < RawInfo > , pub datagram_id : Option < u32 > , pub trigger : Option < PacketBufferedTrigger > , }
};
}
