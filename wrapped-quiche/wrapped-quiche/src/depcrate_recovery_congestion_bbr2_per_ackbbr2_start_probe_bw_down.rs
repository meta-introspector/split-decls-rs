// Generated macro for bbr2_start_probe_bw_down (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_start_probe_bw_down {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_start_probe_bw_down"}
// Dependencies: {}
pub fn bbr2_start_probe_bw_down (r : & mut Congestion , now : Instant) { per_loss :: bbr2_reset_congestion_signals (r) ; r . bbr2_state . probe_up_cnt = usize :: MAX ; bbr2_pick_probe_wait (r) ; r . bbr2_state . cycle_stamp = now ; r . bbr2_state . ack_phase = BBR2AckPhase :: ProbeStopping ; bbr2_start_round (r) ; r . bbr2_state . state = BBR2StateMachine :: ProbeBWDOWN ; r . bbr2_state . pacing_gain = PROBE_DOWN_PACING_GAIN ; r . bbr2_state . cwnd_gain = CWND_GAIN }
};
}
