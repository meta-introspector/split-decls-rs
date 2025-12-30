// Generated macro for bbr_enter_recovery (function)
macro_rules! Depcrate_recovery_congestion_bbrbbr_enter_recovery {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"bbr_enter_recovery"}
// Dependencies: {}
fn bbr_enter_recovery (r : & mut Congestion , in_flight : usize , now : Instant) { r . bbr_state . prior_cwnd = per_ack :: bbr_save_cwnd (r) ; r . congestion_window = in_flight . max (r . max_datagram_size) ; r . congestion_recovery_start_time = Some (now) ; r . bbr_state . packet_conservation = true ; r . bbr_state . in_recovery = true ; r . bbr_state . newly_lost_bytes = 0 ; r . bbr_state . next_round_delivered = r . delivery_rate . delivered () ; }
};
}
