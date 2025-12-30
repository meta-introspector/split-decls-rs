// Generated macro for impl_34 (impl)
macro_rules! Depcrate_parallel_job_tokenimpl_34 {
() => {
// Module: crate::parallel::job_token
// Provides: {"impl_34"}
// Dependencies: {}
impl Drop for JobToken { fn drop (& mut self) { match JobTokenServer :: new () { JobTokenServer :: Inherited (jobserver) => jobserver . release_token_raw () , JobTokenServer :: InProcess (jobserver) => jobserver . release_token_raw () , } } }
};
}
