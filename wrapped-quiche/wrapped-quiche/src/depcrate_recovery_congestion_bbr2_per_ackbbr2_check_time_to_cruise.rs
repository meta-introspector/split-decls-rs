// Generated macro for bbr2_check_time_to_cruise (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_time_to_cruise {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_time_to_cruise"}
// Dependencies: {}
fn bbr2_check_time_to_cruise (r : & mut Congestion , in_flight : usize) -> bool { if in_flight > bbr2_inflight_with_headroom (r) { return false ; } if in_flight <= bbr2_inflight (r , r . bbr2_state . max_bw , 1.0) { return true ; } false }
};
}
