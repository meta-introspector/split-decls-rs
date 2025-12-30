// Generated macro for state_str (function)
macro_rules! Depcrate_recovery_congestion_renostate_str {
() => {
// Module: crate::recovery::congestion::reno
// Provides: {"state_str"}
// Dependencies: {}
# [cfg (feature = "qlog")] pub fn state_str (r : & Congestion , now : Instant) -> & 'static str { if r . hystart . in_css () { "conservative_slow_start" } else if r . congestion_window < r . ssthresh . get () { "slow_start" } else if r . in_congestion_recovery (now) { "recovery" } else { "congestion_avoidance" } }
};
}
