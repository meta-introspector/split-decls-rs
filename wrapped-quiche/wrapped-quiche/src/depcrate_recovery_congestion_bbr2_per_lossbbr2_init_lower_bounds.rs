// Generated macro for bbr2_init_lower_bounds (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_init_lower_bounds {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_init_lower_bounds"}
// Dependencies: {}
fn bbr2_init_lower_bounds (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; if bbr . bw_lo == u64 :: MAX { bbr . bw_lo = bbr . max_bw ; } if bbr . inflight_lo == usize :: MAX { bbr . inflight_lo = r . congestion_window ; } }
};
}
