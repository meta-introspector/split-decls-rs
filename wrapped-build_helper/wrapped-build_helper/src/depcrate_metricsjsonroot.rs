// Generated macro for JsonRoot (struct)
macro_rules! Depcrate_metricsJsonRoot {
() => {
// Module: crate::metrics
// Provides: {"JsonRoot"}
// Dependencies: {}
# [derive (Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub struct JsonRoot { # [serde (default)] pub format_version : usize , pub system_stats : JsonInvocationSystemStats , pub invocations : Vec < JsonInvocation > , # [serde (default)] pub ci_metadata : Option < CiMetadata > , }
};
}
