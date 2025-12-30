// Generated macro for skip_jobs (function)
macro_rules! Depcrate_jobsskip_jobs {
() => {
// Module: crate::jobs
// Provides: {"skip_jobs"}
// Dependencies: {}
# [doc = " Skip CI jobs that are not supposed to be executed on the given `channel`."] fn skip_jobs (jobs : Vec < Job > , channel : & str) -> Vec < Job > { jobs . into_iter () . filter (| job | { job . only_on_channel . is_none () || job . only_on_channel . as_deref () == Some (channel) }) . collect () }
};
}
