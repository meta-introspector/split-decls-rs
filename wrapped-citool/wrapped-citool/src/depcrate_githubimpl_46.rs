// Generated macro for impl_46 (impl)
macro_rules! Depcrate_githubimpl_46 {
() => {
// Module: crate::github
// Provides: {"impl_46"}
// Dependencies: {}
impl JobInfoResolver { pub fn new () -> Self { Self { client : GitHubClient , workflow_job_cache : Default :: default () } } # [doc = " Get a link to a job summary for the given job name and bootstrap execution."] pub fn get_job_summary_link (& mut self , job_name : & str , metrics : & JsonRoot) -> Option < String > { metrics . ci_metadata . as_ref () . and_then (| metadata | { self . get_job_id (metadata , job_name) . map (| job_id | { format ! ("https://github.com/{}/actions/runs/{}#summary-{job_id}" , metadata . repository , metadata . workflow_run_id) }) }) } fn get_job_id (& mut self , ci_metadata : & CiMetadata , job_name : & str) -> Option < u64 > { if let Some (job) = self . workflow_job_cache . get (& ci_metadata . workflow_run_id) . and_then (| jobs | jobs . iter () . find (| j | j . name == job_name)) { return Some (job . id) ; } let jobs = self . client . get_workflow_run_jobs (& ci_metadata . repository , ci_metadata . workflow_run_id) . inspect_err (| e | eprintln ! ("Cannot download workflow jobs: {e:?}")) . ok () ? ; let job_id = jobs . iter () . find (| j | j . name == job_name) . map (| j | j . id) ; self . workflow_job_cache . insert (ci_metadata . workflow_run_id , jobs) ; job_id } }
};
}
