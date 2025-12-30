// Generated macro for bbr_set_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_set_cwnd {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_set_cwnd"}
// Dependencies: {}
fn bbr_set_cwnd (r : & mut Congestion , bytes_in_flight : usize) { let acked_bytes = r . bbr_state . newly_acked_bytes ; bbr_update_target_cwnd (r) ; bbr_modulate_cwnd_for_recovery (r , bytes_in_flight) ; if ! r . bbr_state . packet_conservation { if r . bbr_state . filled_pipe { r . congestion_window = cmp :: min (r . congestion_window + acked_bytes , r . bbr_state . target_cwnd ,) } else if r . congestion_window < r . bbr_state . target_cwnd || r . delivery_rate . delivered () < r . max_datagram_size * r . initial_congestion_window_packets { r . congestion_window += acked_bytes ; } r . congestion_window = r . congestion_window . max (bbr_min_pipe_cwnd (r)) } bbr_modulate_cwnd_for_probe_rtt (r) ; }
};
}
