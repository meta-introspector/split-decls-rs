// Generated macro for JsonStepSystemStats (struct)
macro_rules! Depcrate_metricsJsonStepSystemStats {
() => {
// Module: crate::metrics
// Provides: {"JsonStepSystemStats"}
// Dependencies: {}
# [derive (Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub struct JsonStepSystemStats { # [serde (deserialize_with = "null_as_f64_nan")] pub cpu_utilization_percent : f64 , }
};
}
