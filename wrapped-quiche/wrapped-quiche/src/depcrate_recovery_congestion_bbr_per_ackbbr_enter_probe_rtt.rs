// Generated macro for bbr_enter_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_enter_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_enter_probe_rtt"}
// Dependencies: {}
fn bbr_enter_probe_rtt (r : & mut Congestion) { let bbr = & mut r . bbr_state ; bbr . state = BBRStateMachine :: ProbeRTT ; bbr . pacing_gain = 1.0 ; bbr . cwnd_gain = 1.0 ; }
};
}
