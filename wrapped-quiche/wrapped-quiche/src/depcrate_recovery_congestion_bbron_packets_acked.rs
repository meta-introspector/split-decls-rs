// Generated macro for on_packets_acked (function)
macro_rules! Depcrate_recovery_congestion_bbron_packets_acked {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"on_packets_acked"}
// Dependencies: {}
fn on_packets_acked (r : & mut Congestion , bytes_in_flight : usize , packets : & mut Vec < Acked > , now : Instant , _rtt_stats : & RttStats ,) { r . bbr_state . prior_bytes_in_flight = bytes_in_flight ; r . bbr_state . newly_acked_bytes = packets . drain (..) . fold (0 , | acked_bytes , p | { r . bbr_state . prior_bytes_in_flight -= p . size ; per_ack :: bbr_update_model_and_state (r , & p , bytes_in_flight , now) ; acked_bytes + p . size }) ; if let Some (pkt) = packets . last () { if ! r . in_congestion_recovery (pkt . time_sent) && r . bbr_state . in_recovery { bbr_exit_recovery (r) ; } } per_ack :: bbr_update_control_parameters (r , bytes_in_flight , now) ; r . bbr_state . newly_lost_bytes = 0 ; }
};
}
