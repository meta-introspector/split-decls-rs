// Generated macro for GithubActionsJob (struct)
macro_rules! Depcrate_jobsGithubActionsJob {
() => {
// Module: crate::jobs
// Provides: {"GithubActionsJob"}
// Dependencies: {}
# [doc = " Representation of a job outputted to a GitHub Actions workflow."] # [derive (serde :: Serialize , Debug)] struct GithubActionsJob { # [doc = " The main identifier of the job, used by CI scripts to determine what should be executed."] name : String , # [doc = " Helper label displayed in GitHub Actions interface, containing the job name and a run type"] # [doc = " prefix (PR/try/auto)."] full_name : String , os : String , env : BTreeMap < String , serde_json :: Value > , # [serde (skip_serializing_if = "Option::is_none")] continue_on_error : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] free_disk : Option < bool > , # [serde (skip_serializing_if = "Option::is_none")] doc_url : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] codebuild : Option < bool > , }
};
}
