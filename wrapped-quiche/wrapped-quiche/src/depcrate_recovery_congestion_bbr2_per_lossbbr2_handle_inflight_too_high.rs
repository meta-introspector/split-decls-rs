// Generated macro for bbr2_handle_inflight_too_high (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_handle_inflight_too_high {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_handle_inflight_too_high"}
// Dependencies: {}
fn bbr2_handle_inflight_too_high (r : & mut Congestion , now : Instant) { r . bbr2_state . bw_probe_samples = false ; if ! r . delivery_rate . sample_is_app_limited () { r . bbr2_state . inflight_hi = r . bbr2_state . tx_in_flight . max ((per_ack :: bbr2_target_inflight (r) as f64 * BETA) as usize) ; } if r . bbr2_state . state == BBR2StateMachine :: ProbeBWUP { per_ack :: bbr2_start_probe_bw_down (r , now) ; } }
};
}
