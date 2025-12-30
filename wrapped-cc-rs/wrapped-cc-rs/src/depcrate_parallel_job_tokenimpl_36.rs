// Generated macro for impl_36 (impl)
macro_rules! Depcrate_parallel_job_tokenimpl_36 {
() => {
// Module: crate::parallel::job_token
// Provides: {"impl_36"}
// Dependencies: {}
impl JobTokenServer { # [doc = " This function returns a static reference to the jobserver because"] # [doc = "  - creating a jobserver from env is a bit fd-unsafe (e.g. the fd might"] # [doc = "    be closed by other jobserver users in the process) and better do it"] # [doc = "    at the start of the program."] # [doc = "  - in case a jobserver cannot be created from env (e.g. it's not"] # [doc = "    present), we will create a global in-process only jobserver"] # [doc = "    that has to be static so that it will be shared by all cc"] # [doc = "    compilation."] fn new () -> & 'static Self { static JOBSERVER : OnceLock < JobTokenServer > = OnceLock :: new () ; JOBSERVER . get_or_init (| | { unsafe { inherited_jobserver :: JobServer :: from_env () } . map (Self :: Inherited) . unwrap_or_else (| | Self :: InProcess (inprocess_jobserver :: JobServer :: new ())) }) } }
};
}
