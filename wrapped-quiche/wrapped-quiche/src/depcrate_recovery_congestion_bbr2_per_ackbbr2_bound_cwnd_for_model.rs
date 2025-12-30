// Generated macro for bbr2_bound_cwnd_for_model (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_bound_cwnd_for_model {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_bound_cwnd_for_model"}
// Dependencies: {}
fn bbr2_bound_cwnd_for_model (r : & mut Congestion) { let mut cap = usize :: MAX ; if bbr2_is_in_a_probe_bw_state (r) && r . bbr2_state . state != BBR2StateMachine :: ProbeBWCRUISE { cap = r . bbr2_state . inflight_hi ; } else if r . bbr2_state . state == BBR2StateMachine :: ProbeRTT || r . bbr2_state . state == BBR2StateMachine :: ProbeBWCRUISE { cap = bbr2_inflight_with_headroom (r) ; } cap = cap . min (r . bbr2_state . inflight_lo) ; cap = cap . max (bbr2_min_pipe_cwnd (r)) ; r . congestion_window = r . congestion_window . min (cap) ; }
};
}
