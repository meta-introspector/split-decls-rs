// Generated macro for JsonInvocationSystemStats (struct)
macro_rules! Depcrate_metricsJsonInvocationSystemStats {
() => {
// Module: crate::metrics
// Provides: {"JsonInvocationSystemStats"}
// Dependencies: {}
# [derive (Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub struct JsonInvocationSystemStats { pub cpu_threads_count : usize , pub cpu_model : String , pub memory_total_bytes : u64 , }
};
}
