// Generated macro for bbr2_adapt_lower_bounds_from_congestion (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_adapt_lower_bounds_from_congestion {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_adapt_lower_bounds_from_congestion"}
// Dependencies: {}
fn bbr2_adapt_lower_bounds_from_congestion (r : & mut Congestion) { if bbr2_is_probing_bw (r) { return ; } if r . bbr2_state . loss_in_round { bbr2_init_lower_bounds (r) ; bbr2_loss_lower_bounds (r) ; } }
};
}
