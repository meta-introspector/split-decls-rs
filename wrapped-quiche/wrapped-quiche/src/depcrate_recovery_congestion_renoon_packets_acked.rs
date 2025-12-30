// Generated macro for on_packets_acked (function)
macro_rules! Depcrate_recovery_congestion_renoon_packets_acked {
() => {
// Module: crate::recovery::congestion::reno
// Provides: {"on_packets_acked"}
// Dependencies: {}
fn on_packets_acked (r : & mut Congestion , _bytes_in_flight : usize , packets : & mut Vec < Acked > , now : Instant , rtt_stats : & RttStats ,) { for pkt in packets . drain (..) { on_packet_acked (r , & pkt , now , rtt_stats) ; } }
};
}
