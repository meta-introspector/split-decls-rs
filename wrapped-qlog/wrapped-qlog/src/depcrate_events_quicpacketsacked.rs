// Generated macro for PacketsAcked (struct)
macro_rules! Depcrate_events_quicPacketsAcked {
() => {
// Module: crate::events::quic
// Provides: {"PacketsAcked"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct PacketsAcked { pub packet_number_space : Option < PacketNumberSpace > , pub packet_numbers : Option < Vec < u64 > > , }
};
}
