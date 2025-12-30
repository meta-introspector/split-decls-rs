// Generated macro for bbr_modulate_cwnd_for_recovery (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_modulate_cwnd_for_recovery {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_modulate_cwnd_for_recovery"}
// Dependencies: {}
fn bbr_modulate_cwnd_for_recovery (r : & mut Congestion , bytes_in_flight : usize) { let acked_bytes = r . bbr_state . newly_acked_bytes ; let lost_bytes = r . bbr_state . newly_lost_bytes ; if lost_bytes > 0 { r . congestion_window = r . congestion_window . saturating_sub (lost_bytes) . max (r . max_datagram_size * MINIMUM_WINDOW_PACKETS) ; } if r . bbr_state . packet_conservation { r . congestion_window = r . congestion_window . max (bytes_in_flight + acked_bytes) ; } }
};
}
