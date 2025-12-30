// Generated macro for RunType (enum)
macro_rules! Depcrate_jobsRunType {
() => {
// Module: crate::jobs
// Provides: {"RunType"}
// Dependencies: {}
# [doc = " Type of workflow that is being executed on CI"] # [derive (Debug)] pub enum RunType { # [doc = " Workflows that run after a push to a PR branch"] PullRequest , # [doc = " Try run started with @bors try"] TryJob { job_patterns : Option < Vec < String > > } , # [doc = " Merge attempt workflow"] AutoJob , # [doc = " Fake job only used for sharing Github Actions cache."] MasterJob , }
};
}
