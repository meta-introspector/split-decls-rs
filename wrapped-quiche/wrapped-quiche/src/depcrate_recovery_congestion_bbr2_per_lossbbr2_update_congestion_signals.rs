// Generated macro for bbr2_update_congestion_signals (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_update_congestion_signals {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_update_congestion_signals"}
// Dependencies: {}
pub fn bbr2_update_congestion_signals (r : & mut Congestion , packet : & Acked) { per_ack :: bbr2_update_max_bw (r , packet) ; if r . bbr2_state . lost > 0 { r . bbr2_state . loss_in_round = true ; r . bbr2_state . loss_events_in_round += 1 ; } if ! r . bbr2_state . loss_round_start { return ; } bbr2_adapt_lower_bounds_from_congestion (r) ; r . bbr2_state . loss_in_round = false ; r . bbr2_state . loss_events_in_round = 0 ; }
};
}
