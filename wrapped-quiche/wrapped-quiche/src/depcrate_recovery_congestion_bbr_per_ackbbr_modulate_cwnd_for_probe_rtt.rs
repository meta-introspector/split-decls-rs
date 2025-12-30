// Generated macro for bbr_modulate_cwnd_for_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_modulate_cwnd_for_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_modulate_cwnd_for_probe_rtt"}
// Dependencies: {}
fn bbr_modulate_cwnd_for_probe_rtt (r : & mut Congestion) { if r . bbr_state . state == BBRStateMachine :: ProbeRTT { r . congestion_window = r . congestion_window . min (bbr_min_pipe_cwnd (r)) } }
};
}
