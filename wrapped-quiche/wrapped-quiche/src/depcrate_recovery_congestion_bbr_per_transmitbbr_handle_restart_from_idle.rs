// Generated macro for bbr_handle_restart_from_idle (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_transmitbbr_handle_restart_from_idle {
() => {
// Module: crate::recovery::congestion::bbr::per_transmit
// Provides: {"bbr_handle_restart_from_idle"}
// Dependencies: {}
fn bbr_handle_restart_from_idle (r : & mut Congestion , bytes_in_flight : usize) { if bytes_in_flight == 0 && r . delivery_rate . app_limited () { r . bbr_state . idle_restart = true ; if r . bbr_state . state == BBRStateMachine :: ProbeBW { pacing :: bbr_set_pacing_rate_with_gain (r , 1.0) ; } } }
};
}
