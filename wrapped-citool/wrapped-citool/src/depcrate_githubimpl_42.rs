// Generated macro for impl_42 (impl)
macro_rules! Depcrate_githubimpl_42 {
() => {
// Module: crate::github
// Provides: {"impl_42"}
// Dependencies: {}
impl GitHubClient { fn get_workflow_run_jobs (& self , repo : & str , workflow_run_id : u64 ,) -> anyhow :: Result < Vec < GitHubJob > > { let req = ureq :: get (format ! ("https://api.github.com/repos/{repo}/actions/runs/{workflow_run_id}/jobs?per_page=100")) . header ("User-Agent" , "rust-lang/rust/citool") . header ("Accept" , "application/vnd.github+json") . header ("X-GitHub-Api-Version" , "2022-11-28") . call () . context ("cannot get workflow job list") ? ; let status = req . status () ; let mut body = req . into_body () ; if status . is_success () { let response = body . read_json :: < WorkflowRunJobsResponse > () . context ("cannot deserialize workflow run jobs response") ? ; Ok (response . jobs . into_iter () . map (| mut job | { job . name = job . name . split_once (" - ") . map (| res | res . 1 . to_string ()) . unwrap_or_else (| | job . name) ; job }) . collect ()) } else { Err (anyhow :: anyhow ! ("Cannot get jobs of workflow run {workflow_run_id}: {status}\n{}" , body . read_to_string () ?)) } } }
};
}
