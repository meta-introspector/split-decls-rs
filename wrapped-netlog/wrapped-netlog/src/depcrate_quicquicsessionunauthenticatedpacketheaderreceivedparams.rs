// Generated macro for QuicSessionUnauthenticatedPacketHeaderReceivedParams (struct)
macro_rules! Depcrate_quicQuicSessionUnauthenticatedPacketHeaderReceivedParams {
() => {
// Module: crate::quic
// Provides: {"QuicSessionUnauthenticatedPacketHeaderReceivedParams"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default)] pub struct QuicSessionUnauthenticatedPacketHeaderReceivedParams { pub connection_id : String , pub header_format : String , pub long_header_type : Option < String > , pub packet_number : u64 , }
};
}
