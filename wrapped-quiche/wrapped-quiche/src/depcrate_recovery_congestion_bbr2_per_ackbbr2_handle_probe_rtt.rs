// Generated macro for bbr2_handle_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_handle_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_handle_probe_rtt"}
// Dependencies: {}
fn bbr2_handle_probe_rtt (r : & mut Congestion , in_flight : usize , now : Instant) { r . delivery_rate . update_app_limited (true) ; if r . bbr2_state . probe_rtt_done_stamp . is_some () { if r . bbr2_state . round_start { r . bbr2_state . probe_rtt_round_done = true ; } if r . bbr2_state . probe_rtt_round_done { bbr2_check_probe_rtt_done (r , now) ; } } else if in_flight <= bbr2_probe_rtt_cwnd (r) { r . bbr2_state . probe_rtt_done_stamp = Some (now + PROBE_RTT_DURATION) ; r . bbr2_state . probe_rtt_round_done = false ; bbr2_start_round (r) ; } }
};
}
