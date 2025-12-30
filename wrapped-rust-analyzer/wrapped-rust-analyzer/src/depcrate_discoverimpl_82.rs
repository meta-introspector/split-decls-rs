// Generated macro for impl_82 (impl)
macro_rules! Depcrate_discoverimpl_82 {
() => {
// Module: crate::discover
// Provides: {"impl_82"}
// Dependencies: {}
impl DiscoverProjectMessage { fn new (data : DiscoverProjectData) -> Self { match data { DiscoverProjectData :: Finished { project , buildfile , .. } => { let buildfile = buildfile . try_into () . expect ("Unable to make path absolute") ; DiscoverProjectMessage :: Finished { project , buildfile } } DiscoverProjectData :: Error { error , source } => { DiscoverProjectMessage :: Error { error , source } } DiscoverProjectData :: Progress { message } => { DiscoverProjectMessage :: Progress { message } } } } }
};
}
