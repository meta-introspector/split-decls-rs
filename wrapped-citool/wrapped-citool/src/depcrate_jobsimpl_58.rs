// Generated macro for impl_58 (impl)
macro_rules! Depcrate_jobsimpl_58 {
() => {
// Module: crate::jobs
// Provides: {"impl_58"}
// Dependencies: {}
impl JobDatabase { # [doc = " Find `auto` jobs that correspond to the passed `pattern`."] # [doc = " Patterns are matched using the glob syntax."] # [doc = " For example `dist-*` matches all jobs starting with `dist-`."] fn find_auto_or_optional_jobs_by_pattern (& self , pattern : & str) -> Vec < Job > { self . auto_jobs . iter () . chain (self . optional_jobs . iter ()) . filter (| j | glob_match :: glob_match (pattern , & j . name)) . cloned () . collect () } fn find_auto_job_by_name (& self , job_name : & str) -> Option < & Job > { self . auto_jobs . iter () . find (| job | job . name == job_name) } }
};
}
