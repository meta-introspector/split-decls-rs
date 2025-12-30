// Generated macro for bbr2_loss_lower_bounds (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_loss_lower_bounds {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_loss_lower_bounds"}
// Dependencies: {}
fn bbr2_loss_lower_bounds (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . bw_lo = bbr . bw_latest . max ((bbr . bw_lo as f64 * BETA) as u64) ; bbr . inflight_lo = bbr . inflight_latest . max ((bbr . inflight_lo as f64 * BETA) as usize) ; }
};
}
