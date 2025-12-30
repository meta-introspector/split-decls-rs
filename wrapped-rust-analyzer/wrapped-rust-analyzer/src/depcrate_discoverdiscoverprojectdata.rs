// Generated macro for DiscoverProjectData (enum)
macro_rules! Depcrate_discoverDiscoverProjectData {
() => {
// Module: crate::discover
// Provides: {"DiscoverProjectData"}
// Dependencies: {}
# [doc = " An enum containing either progress messages, an error,"] # [doc = " or the materialized `rust-project`."] # [derive (Debug , Clone , Deserialize , Serialize)] # [serde (tag = "kind")] # [serde (rename_all = "snake_case")] enum DiscoverProjectData { Finished { buildfile : Utf8PathBuf , project : ProjectJsonData } , Error { error : String , source : Option < String > } , Progress { message : String } , }
};
}
