// Generated macro for bbr2_update_probe_bw_cycle_phase (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_probe_bw_cycle_phase {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_probe_bw_cycle_phase"}
// Dependencies: {}
fn bbr2_update_probe_bw_cycle_phase (r : & mut Congestion , in_flight : usize , now : Instant ,) { if ! r . bbr2_state . filled_pipe { return ; } bbr2_adapt_upper_bounds (r , now) ; if ! bbr2_is_in_a_probe_bw_state (r) { return ; } match r . bbr2_state . state { BBR2StateMachine :: ProbeBWDOWN => { if bbr2_check_time_to_probe_bw (r , now) { return ; } if bbr2_check_time_to_cruise (r , in_flight) { bbr2_start_probe_bw_cruise (r) ; } } , BBR2StateMachine :: ProbeBWCRUISE => { bbr2_check_time_to_probe_bw (r , now) ; } , BBR2StateMachine :: ProbeBWREFILL => { if r . bbr2_state . round_start { r . bbr2_state . bw_probe_samples = true ; bbr2_start_probe_bw_up (r , now) ; } } , BBR2StateMachine :: ProbeBWUP => { if bbr2_has_elapsed_in_phase (r , r . bbr2_state . min_rtt , now) && in_flight > bbr2_inflight (r , r . bbr2_state . max_bw , 1.25) { bbr2_start_probe_bw_down (r , now) ; } } , _ => () , } }
};
}
