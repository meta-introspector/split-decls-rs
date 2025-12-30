// Generated macro for PacketDroppedTrigger (enum)
macro_rules! Depcrate_events_quicPacketDroppedTrigger {
() => {
// Module: crate::events::quic
// Provides: {"PacketDroppedTrigger"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum PacketDroppedTrigger { InternalError , Rejected , Unsupported , Invalid , ConnectionUnknown , DecryptionFailure , General , }
};
}
