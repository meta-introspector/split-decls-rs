// Generated macro for PacketSentTrigger (enum)
macro_rules! Depcrate_events_quicPacketSentTrigger {
() => {
// Module: crate::events::quic
// Provides: {"PacketSentTrigger"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum PacketSentTrigger { RetransmitReordered , RetransmitTimeout , PtoProbe , RetransmitCrypto , CcBandwidthProbe , }
};
}
