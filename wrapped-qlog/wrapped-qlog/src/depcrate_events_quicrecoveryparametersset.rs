// Generated macro for RecoveryParametersSet (struct)
macro_rules! Depcrate_events_quicRecoveryParametersSet {
() => {
// Module: crate::events::quic
// Provides: {"RecoveryParametersSet"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug , Default)] pub struct RecoveryParametersSet { pub reordering_threshold : Option < u16 > , pub time_threshold : Option < f32 > , pub timer_granularity : Option < u16 > , pub initial_rtt : Option < f32 > , pub max_datagram_size : Option < u32 > , pub initial_congestion_window : Option < u64 > , pub minimum_congestion_window : Option < u32 > , pub loss_reduction_factor : Option < f32 > , pub persistent_congestion_threshold : Option < u16 > , }
};
}
