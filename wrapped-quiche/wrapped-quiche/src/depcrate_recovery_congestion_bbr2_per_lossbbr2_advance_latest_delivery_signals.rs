// Generated macro for bbr2_advance_latest_delivery_signals (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_advance_latest_delivery_signals {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_advance_latest_delivery_signals"}
// Dependencies: {}
pub fn bbr2_advance_latest_delivery_signals (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; if bbr . loss_round_start { bbr . bw_latest = r . delivery_rate . sample_delivery_rate () . to_bytes_per_second () ; bbr . inflight_latest = r . delivery_rate . sample_delivered () ; } }
};
}
