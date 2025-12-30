// Generated macro for impl_3 (impl)
macro_rules! Depcrate_ciimpl_3 {
() => {
// Module: crate::ci
// Provides: {"impl_3"}
// Dependencies: {}
impl CiEnv { # [doc = " Obtains the current CI environment."] pub fn current () -> CiEnv { if std :: env :: var ("GITHUB_ACTIONS") . is_ok_and (| e | e == "true") { CiEnv :: GitHubActions } else { CiEnv :: None } } pub fn is_ci () -> bool { Self :: current () . is_running_in_ci () } pub fn is_running_in_ci (self) -> bool { self != CiEnv :: None } # [doc = " Checks if running in rust-lang/rust managed CI job."] pub fn is_rust_lang_managed_ci_job () -> bool { Self :: is_ci () && std :: env :: var_os ("CI_JOB_NAME") . is_some () && std :: env :: var_os ("TOOLSTATE_REPO") . is_some () } }
};
}
