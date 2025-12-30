// Generated macro for RttStats (struct)
macro_rules! Depcrate_recovery_rttRttStats {
() => {
// Module: crate::recovery::rtt
// Provides: {"RttStats"}
// Dependencies: {}
pub struct RttStats { pub (super) latest_rtt : Duration , max_rtt : Duration , pub (super) smoothed_rtt : Duration , pub (super) rttvar : Duration , pub (super) min_rtt : Minmax < Duration > , pub (super) max_ack_delay : Duration , pub (super) has_first_rtt_sample : bool , }
};
}
