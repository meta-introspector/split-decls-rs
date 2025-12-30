// Generated macro for impl_87 (impl)
macro_rules! Depcrate_discoverimpl_87 {
() => {
// Module: crate::discover
// Provides: {"impl_87"}
// Dependencies: {}
impl DiscoverProjectMessage { fn new (data : DiscoverProjectData) -> Self { match data { DiscoverProjectData :: Finished { project , buildfile , .. } => { let buildfile = buildfile . try_into () . expect ("Unable to make path absolute") ; DiscoverProjectMessage :: Finished { project , buildfile } } DiscoverProjectData :: Error { error , source } => { DiscoverProjectMessage :: Error { error , source } } DiscoverProjectData :: Progress { message } => { DiscoverProjectMessage :: Progress { message } } } } }
};
}
