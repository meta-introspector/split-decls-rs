// Generated macro for bbr_enter_probe_bw (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_enter_probe_bw {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_enter_probe_bw"}
// Dependencies: {}
fn bbr_enter_probe_bw (r : & mut Congestion , now : Instant) { let bbr = & mut r . bbr_state ; bbr . state = BBRStateMachine :: ProbeBW ; bbr . pacing_gain = 1.0 ; bbr . cwnd_gain = 2.0 ; bbr . cycle_index = BBR_GAIN_CYCLE_LEN - 1 - (rand :: rand_u64_uniform (BBR_GAIN_CYCLE_LEN as u64 - 1) as usize) ; bbr_advance_cycle_phase (r , now) ; }
};
}
