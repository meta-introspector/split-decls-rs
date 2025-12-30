// Generated macro for state_str (function)
macro_rules! Depcrate_recovery_congestion_bbrstate_str {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"state_str"}
// Dependencies: {}
# [cfg (feature = "qlog")] fn state_str (r : & Congestion , _now : Instant) -> & 'static str { r . bbr_state . state . into () }
};
}
