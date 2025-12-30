// Generated macro for bbr2_is_inflight_too_high (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_is_inflight_too_high {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_is_inflight_too_high"}
// Dependencies: {}
pub fn bbr2_is_inflight_too_high (r : & mut Congestion) -> bool { r . bbr2_state . lost > (r . bbr2_state . tx_in_flight as f64 * LOSS_THRESH) as usize }
};
}
