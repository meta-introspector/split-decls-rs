// Generated macro for QlogMetrics (struct)
macro_rules! Depcrate_recoveryQlogMetrics {
() => {
// Module: crate::recovery
// Provides: {"QlogMetrics"}
// Dependencies: {}
# [derive (Default)] # [cfg (feature = "qlog")] struct QlogMetrics { min_rtt : Duration , smoothed_rtt : Duration , latest_rtt : Duration , rttvar : Duration , cwnd : u64 , bytes_in_flight : u64 , ssthresh : Option < u64 > , pacing_rate : u64 , }
};
}
