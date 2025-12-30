// Generated macro for JobInfoResolver (struct)
macro_rules! Depcrate_githubJobInfoResolver {
() => {
// Module: crate::github
// Provides: {"JobInfoResolver"}
// Dependencies: {}
# [doc = " Can be used to resolve information about GitHub Actions jobs."] # [doc = " Caches results internally to avoid too unnecessary GitHub API calls."] pub struct JobInfoResolver { client : GitHubClient , workflow_job_cache : HashMap < u64 , Vec < GitHubJob > > , }
};
}
