// Generated macro for load_job_db (function)
macro_rules! Depcrate_jobsload_job_db {
() => {
// Module: crate::jobs
// Provides: {"load_job_db"}
// Dependencies: {}
pub fn load_job_db (db : & str) -> anyhow :: Result < JobDatabase > { let mut db : Value = serde_yaml :: from_str (db) . context ("failed to parse YAML content") ? ; let apply_merge = | db : & mut Value | -> anyhow :: Result < () > { db . apply_merge () . context ("failed to apply merge keys") } ; apply_merge (& mut db) ? ; apply_merge (& mut db) ? ; let mut db : JobDatabase = serde_yaml :: from_value (db) . context ("failed to parse job database") ? ; register_pr_jobs_as_auto_jobs (& mut db) ? ; validate_job_database (& db) ? ; Ok (db) }
};
}
