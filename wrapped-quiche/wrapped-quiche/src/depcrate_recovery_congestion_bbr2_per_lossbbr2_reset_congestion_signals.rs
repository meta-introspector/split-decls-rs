// Generated macro for bbr2_reset_congestion_signals (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_reset_congestion_signals {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_reset_congestion_signals"}
// Dependencies: {}
pub fn bbr2_reset_congestion_signals (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . loss_in_round = false ; bbr . loss_events_in_round = 0 ; bbr . bw_latest = 0 ; bbr . inflight_latest = 0 ; }
};
}
