// Generated macro for PacketLostTrigger (enum)
macro_rules! Depcrate_events_quicPacketLostTrigger {
() => {
// Module: crate::events::quic
// Provides: {"PacketLostTrigger"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum PacketLostTrigger { ReorderingThreshold , TimeThreshold , PtoExpired , }
};
}
