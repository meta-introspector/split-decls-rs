// Generated macro for load_metrics (function)
macro_rules! Depcrate_metricsload_metrics {
() => {
// Module: crate::metrics
// Provides: {"load_metrics"}
// Dependencies: {}
pub fn load_metrics (path : & Path) -> anyhow :: Result < JsonRoot > { let metrics = std :: fs :: read_to_string (path) . with_context (| | format ! ("Cannot read JSON metrics from {path:?}")) ? ; let metrics : JsonRoot = serde_json :: from_str (& metrics) . with_context (| | format ! ("Cannot deserialize JSON metrics from {path:?}")) ? ; Ok (metrics) }
};
}
