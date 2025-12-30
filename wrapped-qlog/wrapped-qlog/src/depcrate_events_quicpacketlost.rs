// Generated macro for PacketLost (struct)
macro_rules! Depcrate_events_quicPacketLost {
() => {
// Module: crate::events::quic
// Provides: {"PacketLost"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug , Default)] pub struct PacketLost { pub header : Option < PacketHeader > , pub frames : Option < Vec < QuicFrame > > , pub trigger : Option < PacketLostTrigger > , }
};
}
