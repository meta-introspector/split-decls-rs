// Generated macro for load_metrics (function)
macro_rules! Depcrate_metricsload_metrics {
() => {
// Module: crate::metrics
// Provides: {"load_metrics"}
// Dependencies: {}
# [doc = " Loads the metrics of the most recent bootstrap execution from a metrics.json file."] pub fn load_metrics (path : & Utf8Path) -> anyhow :: Result < BuildStep > { let content = std :: fs :: read (path . as_std_path ()) ? ; let mut metrics = serde_json :: from_slice :: < JsonRoot > (& content) ? ; let invocation = metrics . invocations . pop () . ok_or_else (| | anyhow :: anyhow ! ("No bootstrap invocation found in metrics file")) ? ; Ok (BuildStep :: from_invocation (& invocation)) }
};
}
