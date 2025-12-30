// Generated macro for bbr2_update_latest_delivery_signals (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_update_latest_delivery_signals {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_update_latest_delivery_signals"}
// Dependencies: {}
pub fn bbr2_update_latest_delivery_signals (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . loss_round_start = false ; bbr . bw_latest = bbr . bw_latest . max (r . delivery_rate . sample_delivery_rate () . to_bytes_per_second ()) ; bbr . inflight_latest = bbr . inflight_latest . max (r . delivery_rate . sample_delivered ()) ; if r . delivery_rate . sample_prior_delivered () >= bbr . loss_round_delivered { bbr . loss_round_delivered = r . delivery_rate . delivered () ; bbr . loss_round_start = true ; } }
};
}
