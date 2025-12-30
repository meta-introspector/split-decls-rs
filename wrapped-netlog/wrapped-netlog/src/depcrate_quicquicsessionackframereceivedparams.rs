// Generated macro for QuicSessionAckFrameReceivedParams (struct)
macro_rules! Depcrate_quicQuicSessionAckFrameReceivedParams {
() => {
// Module: crate::quic
// Provides: {"QuicSessionAckFrameReceivedParams"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default)] pub struct QuicSessionAckFrameReceivedParams { pub delta_time_largest_observed_us : u64 , pub largest_observed : u64 , pub missing_packets : Vec < u64 > , pub received_packet_times : Vec < u64 > , pub smallest_observed : u64 , }
};
}
