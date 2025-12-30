// Generated macro for bbr2_enter_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_enter_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_enter_probe_rtt"}
// Dependencies: {}
fn bbr2_enter_probe_rtt (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . state = BBR2StateMachine :: ProbeRTT ; bbr . pacing_gain = PACING_GAIN ; bbr . cwnd_gain = PROBE_RTT_CWND_GAIN ; }
};
}
