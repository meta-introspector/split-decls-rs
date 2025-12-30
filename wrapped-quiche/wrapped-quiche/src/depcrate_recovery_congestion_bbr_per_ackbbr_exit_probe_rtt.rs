// Generated macro for bbr_exit_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_exit_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_exit_probe_rtt"}
// Dependencies: {}
fn bbr_exit_probe_rtt (r : & mut Congestion , now : Instant) { if r . bbr_state . filled_pipe { bbr_enter_probe_bw (r , now) ; } else { init :: bbr_enter_startup (r) ; } }
};
}
