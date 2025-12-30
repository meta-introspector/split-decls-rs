// Generated macro for MetricsUpdated (struct)
macro_rules! Depcrate_events_quicMetricsUpdated {
() => {
// Module: crate::events::quic
// Provides: {"MetricsUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug , Default)] pub struct MetricsUpdated { pub min_rtt : Option < f32 > , pub smoothed_rtt : Option < f32 > , pub latest_rtt : Option < f32 > , pub rtt_variance : Option < f32 > , pub pto_count : Option < u16 > , pub congestion_window : Option < u64 > , pub bytes_in_flight : Option < u64 > , pub ssthresh : Option < u64 > , pub packets_in_flight : Option < u64 > , pub pacing_rate : Option < u64 > , }
};
}
