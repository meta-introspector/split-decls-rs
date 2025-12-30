// Generated macro for PacketDropped (struct)
macro_rules! Depcrate_events_quicPacketDropped {
() => {
// Module: crate::events::quic
// Provides: {"PacketDropped"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct PacketDropped { pub header : Option < PacketHeader > , pub raw : Option < RawInfo > , pub datagram_id : Option < u32 > , pub details : Option < String > , pub trigger : Option < PacketDroppedTrigger > , }
};
}
