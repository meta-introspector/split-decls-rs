// Generated macro for Error (enum)
macro_rules! Depcrate_configError {
() => {
// Module: crate::config
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned when failing to initialize the repository configuration."] # [doc = ""] # [doc = " This configuration is on the critical path when opening a repository."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ConfigBoolean (# [from] boolean :: Error) , # [error (transparent)] ConfigUnsigned (# [from] unsigned_integer :: Error) , # [error (transparent)] ConfigTypedString (# [from] key :: GenericErrorWithValue) , # [error (transparent)] RefsNamespace (# [from] refs_namespace :: Error) , # [error ("Cannot handle objects formatted as {:?}" , . name)] UnsupportedObjectFormat { name : BString } , # [error (transparent)] CoreAbbrev (# [from] abbrev :: Error) , # [error ("Could not read configuration file at \"{}\"" , path . display ())] Io { source : std :: io :: Error , path : std :: path :: PathBuf , } , # [error (transparent)] Init (# [from] gix_config :: file :: init :: Error) , # [error (transparent)] ResolveIncludes (# [from] gix_config :: file :: includes :: Error) , # [error (transparent)] FromEnv (# [from] gix_config :: file :: init :: from_env :: Error) , # [error ("The path {path:?} at the 'core.worktree' configuration could not be interpolated")] PathInterpolation { path : BString , source : gix_config :: path :: interpolate :: Error , } , # [error ("{source:?} configuration overrides at open or init time could not be applied.")] ConfigOverrides { # [source] err : overrides :: Error , source : gix_config :: Source , } , }
};
}
