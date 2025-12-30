// Generated macro for PacketReceived (struct)
macro_rules! Depcrate_events_quicPacketReceived {
() => {
// Module: crate::events::quic
// Provides: {"PacketReceived"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug , Default)] pub struct PacketReceived { pub header : PacketHeader , pub is_coalesced : Option < bool > , pub retry_token : Option < Token > , pub stateless_reset_token : Option < StatelessResetToken > , pub supported_versions : Option < Vec < Bytes > > , pub raw : Option < RawInfo > , pub datagram_id : Option < u32 > , pub trigger : Option < PacketReceivedTrigger > , pub frames : Option < Vec < QuicFrame > > , }
};
}
