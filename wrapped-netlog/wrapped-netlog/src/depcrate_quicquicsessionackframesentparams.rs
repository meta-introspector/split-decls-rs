// Generated macro for QuicSessionAckFrameSentParams (struct)
macro_rules! Depcrate_quicQuicSessionAckFrameSentParams {
() => {
// Module: crate::quic
// Provides: {"QuicSessionAckFrameSentParams"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default)] pub struct QuicSessionAckFrameSentParams { pub delta_time_largest_observed_us : u64 , pub largest_observed : u64 , pub missing_packets : Vec < u64 > , pub received_packet_times : Vec < u64 > , pub smallest_observed : u64 , }
};
}
