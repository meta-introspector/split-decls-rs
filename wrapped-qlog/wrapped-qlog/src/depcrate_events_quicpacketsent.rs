// Generated macro for PacketSent (struct)
macro_rules! Depcrate_events_quicPacketSent {
() => {
// Module: crate::events::quic
// Provides: {"PacketSent"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug , Default)] pub struct PacketSent { pub header : PacketHeader , pub is_coalesced : Option < bool > , pub retry_token : Option < Token > , pub stateless_reset_token : Option < StatelessResetToken > , pub supported_versions : Option < Vec < Bytes > > , pub raw : Option < RawInfo > , pub datagram_id : Option < u32 > , pub trigger : Option < PacketSentTrigger > , pub send_at_time : Option < f32 > , pub frames : Option < SmallVec < [QuicFrame ; 1] > > , }
};
}
