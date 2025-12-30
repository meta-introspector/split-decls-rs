// Generated macro for bbr2_check_inflight_too_high (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_check_inflight_too_high {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_check_inflight_too_high"}
// Dependencies: {}
pub fn bbr2_check_inflight_too_high (r : & mut Congestion , now : Instant) -> bool { if bbr2_is_inflight_too_high (r) { if r . bbr2_state . bw_probe_samples { bbr2_handle_inflight_too_high (r , now) ; } return true ; } false }
};
}
