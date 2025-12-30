// Generated macro for bbr2_set_pacing_rate_with_gain (function)
macro_rules! Depcrate_recovery_congestion_bbr2_pacingbbr2_set_pacing_rate_with_gain {
() => {
// Module: crate::recovery::congestion::bbr2::pacing
// Provides: {"bbr2_set_pacing_rate_with_gain"}
// Dependencies: {}
pub fn bbr2_set_pacing_rate_with_gain (r : & mut Congestion , pacing_gain : f64) { let rate = (pacing_gain * r . bbr2_state . bw as f64 * (1.0 - PACING_MARGIN_PERCENT)) as u64 ; if r . bbr2_state . filled_pipe || rate > r . bbr2_state . pacing_rate || r . bbr2_state . pacing_rate == r . bbr2_state . init_pacing_rate { r . bbr2_state . pacing_rate = rate ; } }
};
}
