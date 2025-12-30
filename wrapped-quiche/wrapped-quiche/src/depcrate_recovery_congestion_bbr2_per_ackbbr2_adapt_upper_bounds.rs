// Generated macro for bbr2_adapt_upper_bounds (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_adapt_upper_bounds {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_adapt_upper_bounds"}
// Dependencies: {}
fn bbr2_adapt_upper_bounds (r : & mut Congestion , now : Instant) { if r . bbr2_state . ack_phase == BBR2AckPhase :: ProbeStarting && r . bbr2_state . round_start { r . bbr2_state . ack_phase = BBR2AckPhase :: ProbeFeedback ; } if r . bbr2_state . ack_phase == BBR2AckPhase :: ProbeStopping && r . bbr2_state . round_start { r . bbr2_state . bw_probe_samples = false ; r . bbr2_state . ack_phase = BBR2AckPhase :: Init ; if bbr2_is_in_a_probe_bw_state (r) && ! r . delivery_rate . sample_is_app_limited () { bbr2_advance_max_bw_filter (r) ; } } if ! per_loss :: bbr2_check_inflight_too_high (r , now) { if r . bbr2_state . inflight_hi == usize :: MAX || r . bbr2_state . bw_hi == u64 :: MAX { return ; } if r . bbr2_state . tx_in_flight > r . bbr2_state . inflight_hi { r . bbr2_state . inflight_hi = r . bbr2_state . tx_in_flight ; } let delivery_rate = r . delivery_rate () . to_bytes_per_second () ; if delivery_rate > r . bbr2_state . bw_hi { r . bbr2_state . bw_hi = delivery_rate ; } if r . bbr2_state . state == BBR2StateMachine :: ProbeBWUP { bbr2_probe_inflight_hi_upward (r) ; } } }
};
}
