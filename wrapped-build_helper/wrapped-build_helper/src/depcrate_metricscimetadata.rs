// Generated macro for CiMetadata (struct)
macro_rules! Depcrate_metricsCiMetadata {
() => {
// Module: crate::metrics
// Provides: {"CiMetadata"}
// Dependencies: {}
# [doc = " Represents metadata about bootstrap's execution in CI."] # [derive (Serialize , Deserialize)] pub struct CiMetadata { # [doc = " GitHub run ID of the workflow where bootstrap was executed."] # [doc = " Note that the run ID will be shared amongst all jobs executed in that workflow."] pub workflow_run_id : u64 , # [doc = " Full name of a GitHub repository where bootstrap was executed in CI."] # [doc = " e.g. `rust-lang-ci/rust`."] pub repository : String , }
};
}
