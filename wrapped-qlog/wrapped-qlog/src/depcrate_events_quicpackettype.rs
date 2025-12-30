// Generated macro for PacketType (enum)
macro_rules! Depcrate_events_quicPacketType {
() => {
// Module: crate::events::quic
// Provides: {"PacketType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] # [serde (rename_all = "snake_case")] pub enum PacketType { Initial , Handshake , # [serde (rename = "0RTT")] ZeroRtt , # [serde (rename = "1RTT")] OneRtt , Retry , VersionNegotiation , # [default] Unknown , }
};
}
