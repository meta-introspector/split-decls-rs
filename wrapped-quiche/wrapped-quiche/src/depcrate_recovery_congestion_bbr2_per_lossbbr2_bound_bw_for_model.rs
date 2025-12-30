// Generated macro for bbr2_bound_bw_for_model (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_bound_bw_for_model {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_bound_bw_for_model"}
// Dependencies: {}
pub fn bbr2_bound_bw_for_model (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . bw = bbr . max_bw . min (bbr . bw_lo . min (bbr . bw_hi)) ; }
};
}
