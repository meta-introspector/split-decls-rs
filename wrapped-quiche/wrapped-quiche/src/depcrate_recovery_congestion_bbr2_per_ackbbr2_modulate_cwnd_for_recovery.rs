// Generated macro for bbr2_modulate_cwnd_for_recovery (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_modulate_cwnd_for_recovery {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_modulate_cwnd_for_recovery"}
// Dependencies: {}
fn bbr2_modulate_cwnd_for_recovery (r : & mut Congestion , in_flight : usize) { let acked_bytes = r . bbr2_state . newly_acked_bytes ; let lost_bytes = r . bbr2_state . newly_lost_bytes ; if lost_bytes > 0 { r . congestion_window = r . congestion_window . saturating_sub (lost_bytes) . max (r . max_datagram_size * MINIMUM_WINDOW_PACKETS) ; } if r . bbr2_state . packet_conservation { r . congestion_window = r . congestion_window . max (in_flight + acked_bytes) ; } }
};
}
