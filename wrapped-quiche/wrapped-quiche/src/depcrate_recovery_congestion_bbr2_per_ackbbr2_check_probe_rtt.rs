// Generated macro for bbr2_check_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_probe_rtt"}
// Dependencies: {}
fn bbr2_check_probe_rtt (r : & mut Congestion , in_flight : usize , now : Instant) { if r . bbr2_state . state != BBR2StateMachine :: ProbeRTT && r . bbr2_state . probe_rtt_expired && ! r . bbr2_state . idle_restart { bbr2_enter_probe_rtt (r) ; r . bbr2_state . prior_cwnd = bbr2_save_cwnd (r) ; r . bbr2_state . probe_rtt_done_stamp = None ; r . bbr2_state . ack_phase = BBR2AckPhase :: ProbeStopping ; bbr2_start_round (r) ; } if r . bbr2_state . state == BBR2StateMachine :: ProbeRTT { bbr2_handle_probe_rtt (r , in_flight , now) ; } if r . delivery_rate . sample_delivered () > 0 { r . bbr2_state . idle_restart = false ; } }
};
}
