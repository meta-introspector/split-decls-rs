// Generated macro for bbr_handle_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_handle_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_handle_probe_rtt"}
// Dependencies: {}
fn bbr_handle_probe_rtt (r : & mut Congestion , bytes_in_flight : usize , now : Instant ,) { r . delivery_rate . update_app_limited (true) ; if let Some (probe_rtt_done_stamp) = r . bbr_state . probe_rtt_done_stamp { if r . bbr_state . round_start { r . bbr_state . probe_rtt_round_done = true ; } if r . bbr_state . probe_rtt_round_done && now > probe_rtt_done_stamp { r . bbr_state . rtprop_stamp = now ; bbr_restore_cwnd (r) ; bbr_exit_probe_rtt (r , now) ; } } else if bytes_in_flight <= bbr_min_pipe_cwnd (r) { r . bbr_state . probe_rtt_done_stamp = Some (now + PROBE_RTT_DURATION) ; r . bbr_state . probe_rtt_round_done = false ; r . bbr_state . next_round_delivered = r . delivery_rate . delivered () ; } }
};
}
