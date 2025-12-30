// Generated macro for impl_137 (impl)
macro_rules! Depcrateimpl_137 {
() => {
// Module: crate
// Provides: {"impl_137"}
// Dependencies: {}
impl GitHubContext { fn get_run_type (& self) -> Option < RunType > { match (self . event_name . as_str () , self . branch_ref . as_str ()) { ("pull_request" , _) => Some (RunType :: PullRequest) , ("push" , "refs/heads/try-perf") => Some (RunType :: TryJob { job_patterns : None }) , ("push" , "refs/heads/try" | "refs/heads/automation/bors/try") => { let patterns = self . get_try_job_patterns () ; let patterns = if ! patterns . is_empty () { Some (patterns) } else { None } ; Some (RunType :: TryJob { job_patterns : patterns }) } ("push" , "refs/heads/auto") => Some (RunType :: AutoJob) , ("push" , "refs/heads/master") => Some (RunType :: MasterJob) , _ => None , } } # [doc = " Tries to parse patterns of CI jobs that should be executed"] # [doc = " from the commit message of the passed GitHub context"] # [doc = ""] # [doc = " They can be specified in the form of"] # [doc = " try-job: <job-pattern>"] # [doc = " or"] # [doc = " try-job: `<job-pattern>`"] # [doc = " (to avoid GitHub rendering the glob patterns as Markdown)"] fn get_try_job_patterns (& self) -> Vec < String > { if let Some (ref msg) = self . commit_message { msg . lines () . filter_map (| line | line . trim () . strip_prefix ("try-job: ")) . map (| l | l . trim_matches ('`')) . map (| l | l . trim () . to_string ()) . collect () } else { vec ! [] } } }
};
}
