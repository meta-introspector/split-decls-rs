// Generated macro for bbr2_set_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_set_cwnd {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_set_cwnd"}
// Dependencies: {}
fn bbr2_set_cwnd (r : & mut Congestion , in_flight : usize) { let acked_bytes = r . bbr2_state . newly_acked_bytes ; bbr2_update_max_inflight (r) ; bbr2_modulate_cwnd_for_recovery (r , in_flight) ; if ! r . bbr2_state . packet_conservation { if r . bbr2_state . filled_pipe { r . congestion_window = cmp :: min (r . congestion_window + acked_bytes , r . bbr2_state . max_inflight ,) } else if r . congestion_window < r . bbr2_state . max_inflight || r . delivery_rate . delivered () < r . max_datagram_size * r . initial_congestion_window_packets { r . congestion_window += acked_bytes ; } r . congestion_window = r . congestion_window . max (bbr2_min_pipe_cwnd (r)) } bbr2_bound_cwnd_for_probe_rtt (r) ; bbr2_bound_cwnd_for_model (r) ; }
};
}
