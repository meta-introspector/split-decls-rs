// Generated macro for bbr_set_pacing_rate_with_gain (function)
macro_rules! Depcrate_recovery_congestion_bbr_pacingbbr_set_pacing_rate_with_gain {
() => {
// Module: crate::recovery::congestion::bbr::pacing
// Provides: {"bbr_set_pacing_rate_with_gain"}
// Dependencies: {}
pub fn bbr_set_pacing_rate_with_gain (r : & mut Congestion , pacing_gain : f64) { let rate = (pacing_gain * r . bbr_state . btlbw as f64) as u64 ; if r . bbr_state . filled_pipe || rate > r . bbr_state . pacing_rate { r . bbr_state . pacing_rate = rate ; } }
};
}
