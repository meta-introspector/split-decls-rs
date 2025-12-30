// Generated macro for bbr2_start_probe_bw_up (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_start_probe_bw_up {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_start_probe_bw_up"}
// Dependencies: {}
fn bbr2_start_probe_bw_up (r : & mut Congestion , now : Instant) { r . bbr2_state . ack_phase = BBR2AckPhase :: ProbeStarting ; bbr2_start_round (r) ; r . bbr2_state . cycle_stamp = now ; r . bbr2_state . state = BBR2StateMachine :: ProbeBWUP ; r . bbr2_state . pacing_gain = PROBE_UP_PACING_GAIN ; r . bbr2_state . cwnd_gain = CWND_GAIN ; bbr2_raise_inflight_hi_slope (r) ; }
};
}
