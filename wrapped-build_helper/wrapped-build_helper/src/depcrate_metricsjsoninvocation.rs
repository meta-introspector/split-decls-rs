// Generated macro for JsonInvocation (struct)
macro_rules! Depcrate_metricsJsonInvocation {
() => {
// Module: crate::metrics
// Provides: {"JsonInvocation"}
// Dependencies: {}
# [derive (Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub struct JsonInvocation { pub cmdline : String , pub start_time : u64 , # [serde (deserialize_with = "null_as_f64_nan")] pub duration_including_children_sec : f64 , pub children : Vec < JsonNode > , }
};
}
