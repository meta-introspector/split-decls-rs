// Generated macro for bbr2_is_reno_coexistence_probe_time (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_is_reno_coexistence_probe_time {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_is_reno_coexistence_probe_time"}
// Dependencies: {}
fn bbr2_is_reno_coexistence_probe_time (r : & mut Congestion) -> bool { let reno_rounds = bbr2_target_inflight (r) ; let rounds = reno_rounds . min (63) ; r . bbr2_state . rounds_since_probe >= rounds }
};
}
