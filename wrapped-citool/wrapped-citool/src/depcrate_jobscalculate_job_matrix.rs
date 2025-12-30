// Generated macro for calculate_job_matrix (function)
macro_rules! Depcrate_jobscalculate_job_matrix {
() => {
// Module: crate::jobs
// Provides: {"calculate_job_matrix"}
// Dependencies: {}
pub fn calculate_job_matrix (db : JobDatabase , gh_ctx : GitHubContext , channel : & str ,) -> anyhow :: Result < () > { let run_type = gh_ctx . get_run_type () . ok_or_else (| | { anyhow :: anyhow ! ("Cannot determine the type of workflow that is being executed") }) ? ; eprintln ! ("Run type: {run_type:?}") ; let jobs = calculate_jobs (& run_type , & db , channel) ? ; if jobs . is_empty () && ! matches ! (run_type , RunType :: MasterJob) { return Err (anyhow :: anyhow ! ("Computed job list is empty")) ; } let run_type = match run_type { RunType :: PullRequest => "pr" , RunType :: TryJob { .. } => "try" , RunType :: AutoJob => "auto" , RunType :: MasterJob => "master" , } ; eprintln ! ("Output") ; eprintln ! ("jobs={jobs:?}") ; eprintln ! ("run_type={run_type}") ; println ! ("jobs={}" , serde_json :: to_string (& jobs) ?) ; println ! ("run_type={run_type}") ; Ok (()) }
};
}
