// Generated macro for initial_pacing_rate (function)
macro_rules! Depcrate_recovery_gcongestion_bbr2initial_pacing_rate {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"initial_pacing_rate"}
// Dependencies: {}
fn initial_pacing_rate (cwnd_in_bytes : usize , smoothed_rtt : Duration , params : & Params ,) -> Bandwidth { if let Some (pacing_rate) = params . initial_pacing_rate_bytes_per_second { return Bandwidth :: from_bytes_per_second (pacing_rate) ; } Bandwidth :: from_bytes_and_time_delta (cwnd_in_bytes , smoothed_rtt) * 2.885 }
};
}
