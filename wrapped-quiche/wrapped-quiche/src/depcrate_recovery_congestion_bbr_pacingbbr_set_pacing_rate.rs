// Generated macro for bbr_set_pacing_rate (function)
macro_rules! Depcrate_recovery_congestion_bbr_pacingbbr_set_pacing_rate {
() => {
// Module: crate::recovery::congestion::bbr::pacing
// Provides: {"bbr_set_pacing_rate"}
// Dependencies: {}
pub fn bbr_set_pacing_rate (r : & mut Congestion) { bbr_set_pacing_rate_with_gain (r , r . bbr_state . pacing_gain) ; }
};
}
