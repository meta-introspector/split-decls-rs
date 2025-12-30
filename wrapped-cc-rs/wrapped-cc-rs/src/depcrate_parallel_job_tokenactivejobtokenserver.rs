// Generated macro for ActiveJobTokenServer (enum)
macro_rules! Depcrate_parallel_job_tokenActiveJobTokenServer {
() => {
// Module: crate::parallel::job_token
// Provides: {"ActiveJobTokenServer"}
// Dependencies: {}
pub (crate) enum ActiveJobTokenServer { Inherited (inherited_jobserver :: ActiveJobServer < 'static >) , InProcess (& 'static inprocess_jobserver :: JobServer) , }
};
}
