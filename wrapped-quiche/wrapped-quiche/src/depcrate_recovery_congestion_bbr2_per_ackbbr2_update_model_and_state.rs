// Generated macro for bbr2_update_model_and_state (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_model_and_state {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_model_and_state"}
// Dependencies: {}
pub fn bbr2_update_model_and_state (r : & mut Congestion , packet : & Acked , in_flight : usize , now : Instant ,) { per_loss :: bbr2_update_latest_delivery_signals (r) ; per_loss :: bbr2_update_congestion_signals (r , packet) ; bbr2_update_ack_aggregation (r , packet , now) ; bbr2_check_startup_done (r) ; bbr2_check_drain (r , in_flight , now) ; bbr2_update_probe_bw_cycle_phase (r , in_flight , now) ; bbr2_update_min_rtt (r , now) ; bbr2_check_probe_rtt (r , in_flight , now) ; per_loss :: bbr2_advance_latest_delivery_signals (r) ; per_loss :: bbr2_bound_bw_for_model (r) ; }
};
}
