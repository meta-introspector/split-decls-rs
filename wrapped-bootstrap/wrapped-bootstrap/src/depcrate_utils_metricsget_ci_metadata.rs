// Generated macro for get_ci_metadata (function)
macro_rules! Depcrate_utils_metricsget_ci_metadata {
() => {
// Module: crate::utils::metrics
// Provides: {"get_ci_metadata"}
// Dependencies: {}
fn get_ci_metadata (ci_env : CiEnv) -> Option < CiMetadata > { if ci_env != CiEnv :: GitHubActions { return None ; } let workflow_run_id = std :: env :: var ("GITHUB_WORKFLOW_RUN_ID") . ok () . and_then (| id | id . parse :: < u64 > () . ok ()) ? ; let repository = std :: env :: var ("GITHUB_REPOSITORY") . ok () ? ; Some (CiMetadata { workflow_run_id , repository }) }
};
}
