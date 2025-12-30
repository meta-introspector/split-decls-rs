// Generated macro for BuildDataProgress (enum)
macro_rules! Depcrate_reloadBuildDataProgress {
() => {
// Module: crate::reload
// Provides: {"BuildDataProgress"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum BuildDataProgress { Begin , Report (String) , End ((Arc < Vec < ProjectWorkspace > > , Vec < anyhow :: Result < WorkspaceBuildScripts > >)) , }
};
}
