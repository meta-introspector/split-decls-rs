// Generated macro for JobEnvironments (struct)
macro_rules! Depcrate_jobsJobEnvironments {
() => {
// Module: crate::jobs
// Provides: {"JobEnvironments"}
// Dependencies: {}
# [derive (serde :: Deserialize , Debug)] struct JobEnvironments { # [serde (rename = "pr")] pr_env : BTreeMap < String , Value > , # [serde (rename = "try")] try_env : BTreeMap < String , Value > , # [serde (rename = "auto")] auto_env : BTreeMap < String , Value > , }
};
}
