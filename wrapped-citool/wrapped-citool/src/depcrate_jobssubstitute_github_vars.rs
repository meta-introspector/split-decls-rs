// Generated macro for substitute_github_vars (function)
macro_rules! Depcrate_jobssubstitute_github_vars {
() => {
// Module: crate::jobs
// Provides: {"substitute_github_vars"}
// Dependencies: {}
# [doc = " Replace GitHub context variables with environment variables in job configs."] # [doc = " Used for codebuild jobs like"] # [doc = " `codebuild-ubuntu-22-8c-$github.run_id-$github.run_attempt`"] fn substitute_github_vars (jobs : Vec < Job >) -> anyhow :: Result < Vec < Job > > { let run_id = load_env_var ("GITHUB_RUN_ID") ? ; let run_attempt = load_env_var ("GITHUB_RUN_ATTEMPT") ? ; let jobs = jobs . into_iter () . map (| mut job | { job . os = job . os . replace ("$github.run_id" , & run_id) . replace ("$github.run_attempt" , & run_attempt) ; job }) . collect () ; Ok (jobs) }
};
}
