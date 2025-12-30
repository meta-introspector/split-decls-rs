// Generated macro for bbr2_exit_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_exit_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_exit_probe_rtt"}
// Dependencies: {}
fn bbr2_exit_probe_rtt (r : & mut Congestion , now : Instant) { per_loss :: bbr2_reset_lower_bounds (r) ; if r . bbr2_state . filled_pipe { bbr2_start_probe_bw_down (r , now) ; bbr2_start_probe_bw_cruise (r) ; } else { init :: bbr2_enter_startup (r) ; } }
};
}
