// Generated macro for FetchBuildDataResponse (struct)
macro_rules! Depcrate_global_stateFetchBuildDataResponse {
() => {
// Module: crate::global_state
// Provides: {"FetchBuildDataResponse"}
// Dependencies: {}
pub (crate) struct FetchBuildDataResponse { pub (crate) workspaces : Arc < Vec < ProjectWorkspace > > , pub (crate) build_scripts : Vec < anyhow :: Result < WorkspaceBuildScripts > > , }
};
}
