// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type used by this crate."] pub enum Error { NotFound (PathBuf) , CargoManifestDirNotSet , FailedGettingWorkspaceManifestPath , CouldNotRead { path : PathBuf , source : io :: Error } , InvalidToml { source : TomlError } , CrateNotFound { crate_name : String , path : PathBuf } , }
};
}
