// Generated macro for PacketNumberSpace (enum)
macro_rules! Depcrate_events_quicPacketNumberSpace {
() => {
// Module: crate::events::quic
// Provides: {"PacketNumberSpace"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum PacketNumberSpace { Initial , Handshake , ApplicationData , }
};
}
