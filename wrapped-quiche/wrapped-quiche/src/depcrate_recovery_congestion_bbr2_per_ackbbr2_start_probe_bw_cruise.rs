// Generated macro for bbr2_start_probe_bw_cruise (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_start_probe_bw_cruise {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_start_probe_bw_cruise"}
// Dependencies: {}
fn bbr2_start_probe_bw_cruise (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . state = BBR2StateMachine :: ProbeBWCRUISE ; bbr . pacing_gain = PACING_GAIN ; bbr . cwnd_gain = CWND_GAIN ; }
};
}
