// Generated macro for BBRv2 (struct)
macro_rules! Depcrate_recovery_gcongestion_bbr2BBRv2 {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"BBRv2"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct BBRv2 { mode : Mode , cwnd : usize , mss : usize , pacing_rate : Bandwidth , cwnd_limits : Limits < usize > , initial_cwnd : usize , last_sample_is_app_limited : bool , has_non_app_limited_sample : bool , last_quiescence_start : Option < Instant > , params : Params , }
};
}
