// Generated macro for bbr2_bound_cwnd_for_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_bound_cwnd_for_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_bound_cwnd_for_probe_rtt"}
// Dependencies: {}
fn bbr2_bound_cwnd_for_probe_rtt (r : & mut Congestion) { if r . bbr2_state . state == BBR2StateMachine :: ProbeRTT { r . congestion_window = r . congestion_window . min (bbr2_probe_rtt_cwnd (r)) ; } }
};
}
