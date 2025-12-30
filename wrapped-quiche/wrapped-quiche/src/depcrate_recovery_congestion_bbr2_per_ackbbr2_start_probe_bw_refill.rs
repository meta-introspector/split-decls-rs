// Generated macro for bbr2_start_probe_bw_refill (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_start_probe_bw_refill {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_start_probe_bw_refill"}
// Dependencies: {}
fn bbr2_start_probe_bw_refill (r : & mut Congestion) { per_loss :: bbr2_reset_lower_bounds (r) ; r . bbr2_state . bw_probe_up_rounds = 0 ; r . bbr2_state . bw_probe_up_acks = 0 ; r . bbr2_state . ack_phase = BBR2AckPhase :: Refilling ; bbr2_start_round (r) ; r . bbr2_state . state = BBR2StateMachine :: ProbeBWREFILL ; r . bbr2_state . pacing_gain = PACING_GAIN ; r . bbr2_state . cwnd_gain = CWND_GAIN ; }
};
}
