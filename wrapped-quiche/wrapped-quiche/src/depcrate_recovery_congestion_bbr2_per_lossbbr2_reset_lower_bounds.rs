// Generated macro for bbr2_reset_lower_bounds (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_reset_lower_bounds {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_reset_lower_bounds"}
// Dependencies: {}
pub fn bbr2_reset_lower_bounds (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . bw_lo = u64 :: MAX ; bbr . inflight_lo = usize :: MAX ; }
};
}
