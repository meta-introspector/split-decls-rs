// Generated macro for DiscoverProjectMessage (enum)
macro_rules! Depcrate_discoverDiscoverProjectMessage {
() => {
// Module: crate::discover
// Provides: {"DiscoverProjectMessage"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone)] pub (crate) enum DiscoverProjectMessage { Finished { project : ProjectJsonData , buildfile : AbsPathBuf } , Error { error : String , source : Option < String > } , Progress { message : String } , }
};
}
