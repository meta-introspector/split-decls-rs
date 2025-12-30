// Generated macro for bbr2_check_time_to_probe_bw (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_time_to_probe_bw {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_time_to_probe_bw"}
// Dependencies: {}
fn bbr2_check_time_to_probe_bw (r : & mut Congestion , now : Instant) -> bool { if bbr2_has_elapsed_in_phase (r , r . bbr2_state . bw_probe_wait , now) || bbr2_is_reno_coexistence_probe_time (r) { bbr2_start_probe_bw_refill (r) ; return true ; } false }
};
}
