// Generated macro for register_pr_jobs_as_auto_jobs (function)
macro_rules! Depcrate_jobsregister_pr_jobs_as_auto_jobs {
() => {
// Module: crate::jobs
// Provides: {"register_pr_jobs_as_auto_jobs"}
// Dependencies: {}
# [doc = " Maintain invariant that PR CI jobs must be a subset of Auto CI jobs modulo carve-outs."] # [doc = ""] # [doc = " When PR jobs are auto-registered as Auto jobs, they will have `continue_on_error` overridden to"] # [doc = " be `false` to avoid wasting Auto CI resources."] # [doc = ""] # [doc = " When a job is already both a PR job and a auto job, we will post-validate their \"equivalence"] # [doc = " modulo certain carve-outs\" in [`validate_job_database`]."] # [doc = ""] # [doc = " This invariant is important to make sure that it's not easily possible (without modifying"] # [doc = " `citool`) to have PRs with red PR-only CI jobs merged into `master`, causing all subsequent PR"] # [doc = " CI runs to be red until the cause is fixed."] fn register_pr_jobs_as_auto_jobs (db : & mut JobDatabase) -> anyhow :: Result < () > { for pr_job in & db . pr_jobs { if db . find_auto_job_by_name (& pr_job . name) . is_some () { continue ; } let auto_registered_job = Job { continue_on_error : Some (false) , .. pr_job . clone () } ; db . auto_jobs . push (auto_registered_job) ; } Ok (()) }
};
}
