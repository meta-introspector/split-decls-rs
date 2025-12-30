// Generated macro for Job (struct)
macro_rules! Depcrate_jobsJob {
() => {
// Module: crate::jobs
// Provides: {"Job"}
// Dependencies: {}
# [doc = " Representation of a job loaded from the `src/ci/github-actions/jobs.yml` file."] # [derive (serde :: Deserialize , Debug , Clone)] # [serde (deny_unknown_fields)] pub struct Job { # [doc = " Name of the job, e.g. pr-check-1"] pub name : String , # [doc = " GitHub runner on which the job should be executed"] pub os : String , pub env : BTreeMap < String , Value > , # [doc = " Should the job be only executed on a specific channel?"] # [serde (default)] pub only_on_channel : Option < String > , # [doc = " Do not cancel the whole workflow if this job fails."] # [serde (default)] pub continue_on_error : Option < bool > , # [doc = " Free additional disk space in the job, by removing unused packages."] # [serde (default)] pub free_disk : Option < bool > , # [doc = " Documentation link to a resource that could help people debug this CI job."] pub doc_url : Option < String > , # [doc = " Whether the job is executed on AWS CodeBuild."] pub codebuild : Option < bool > , }
};
}
