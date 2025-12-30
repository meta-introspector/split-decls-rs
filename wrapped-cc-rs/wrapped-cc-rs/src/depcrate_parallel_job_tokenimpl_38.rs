// Generated macro for impl_38 (impl)
macro_rules! Depcrate_parallel_job_tokenimpl_38 {
() => {
// Module: crate::parallel::job_token
// Provides: {"impl_38"}
// Dependencies: {}
impl ActiveJobTokenServer { pub (crate) fn new () -> Self { match JobTokenServer :: new () { JobTokenServer :: Inherited (inherited_jobserver) => { Self :: Inherited (inherited_jobserver . enter_active ()) } JobTokenServer :: InProcess (inprocess_jobserver) => Self :: InProcess (inprocess_jobserver) , } } pub (crate) async fn acquire (& mut self) -> Result < JobToken , Error > { match self { Self :: Inherited (jobserver) => jobserver . acquire () . await , Self :: InProcess (jobserver) => Ok (jobserver . acquire () . await) , } } }
};
}
