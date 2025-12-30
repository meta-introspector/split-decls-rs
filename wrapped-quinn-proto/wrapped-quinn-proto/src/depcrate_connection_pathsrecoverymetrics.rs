// Generated macro for RecoveryMetrics (struct)
macro_rules! Depcrate_connection_pathsRecoveryMetrics {
() => {
// Module: crate::connection::paths
// Provides: {"RecoveryMetrics"}
// Dependencies: {}
# [doc = " Congestion metrics as described in [`recovery_metrics_updated`]."] # [doc = ""] # [doc = " [`recovery_metrics_updated`]: https://datatracker.ietf.org/doc/html/draft-ietf-quic-qlog-quic-events.html#name-recovery_metrics_updated"] # [cfg (feature = "qlog")] # [derive (Default , Clone , PartialEq)] # [non_exhaustive] struct RecoveryMetrics { pub min_rtt : Option < Duration > , pub smoothed_rtt : Option < Duration > , pub latest_rtt : Option < Duration > , pub rtt_variance : Option < Duration > , pub pto_count : Option < u32 > , pub bytes_in_flight : Option < u64 > , pub packets_in_flight : Option < u64 > , pub congestion_window : Option < u64 > , pub ssthresh : Option < u64 > , pub pacing_rate : Option < u64 > , }
};
}
