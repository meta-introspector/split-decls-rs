// Generated macro for JobDatabase (struct)
macro_rules! Depcrate_jobsJobDatabase {
() => {
// Module: crate::jobs
// Provides: {"JobDatabase"}
// Dependencies: {}
# [derive (serde :: Deserialize , Debug)] pub struct JobDatabase { # [serde (rename = "pr")] pub pr_jobs : Vec < Job > , # [serde (rename = "try")] pub try_jobs : Vec < Job > , # [serde (rename = "auto")] pub auto_jobs : Vec < Job > , # [serde (rename = "optional")] pub optional_jobs : Vec < Job > , # [doc = " Shared environments for the individual run types."] envs : JobEnvironments , }
};
}
