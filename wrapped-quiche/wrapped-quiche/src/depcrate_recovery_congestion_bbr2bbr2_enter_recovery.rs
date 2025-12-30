// Generated macro for bbr2_enter_recovery (function)
macro_rules! Depcrate_recovery_congestion_bbr2bbr2_enter_recovery {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"bbr2_enter_recovery"}
// Dependencies: {}
fn bbr2_enter_recovery (r : & mut Congestion , in_flight : usize , now : Instant) { r . bbr2_state . prior_cwnd = per_ack :: bbr2_save_cwnd (r) ; r . congestion_window = in_flight + r . bbr2_state . newly_acked_bytes . max (r . max_datagram_size) ; r . congestion_recovery_start_time = Some (now) ; r . bbr2_state . packet_conservation = true ; r . bbr2_state . in_recovery = true ; r . bbr2_state . next_round_delivered = r . delivery_rate . delivered () ; }
};
}
