// Generated macro for on_packets_acked (function)
macro_rules! Depcrate_recovery_congestion_bbr2on_packets_acked {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"on_packets_acked"}
// Dependencies: {}
fn on_packets_acked (r : & mut Congestion , bytes_in_flight : usize , packets : & mut Vec < Acked > , now : Instant , _rtt_stats : & RttStats ,) { r . bbr2_state . newly_acked_bytes = 0 ; let time_sent = packets . last () . map (| pkt | pkt . time_sent) ; r . bbr2_state . prior_bytes_in_flight = bytes_in_flight ; let mut bytes_in_flight = bytes_in_flight ; for p in packets . drain (..) { per_ack :: bbr2_update_model_and_state (r , & p , bytes_in_flight , now) ; r . bbr2_state . prior_bytes_in_flight = bytes_in_flight ; bytes_in_flight -= p . size ; r . bbr2_state . newly_acked_bytes += p . size ; } if let Some (ts) = time_sent { if ! r . in_congestion_recovery (ts) { bbr2_exit_recovery (r) ; } } per_ack :: bbr2_update_control_parameters (r , bytes_in_flight , now) ; r . bbr2_state . newly_lost_bytes = 0 ; }
};
}
