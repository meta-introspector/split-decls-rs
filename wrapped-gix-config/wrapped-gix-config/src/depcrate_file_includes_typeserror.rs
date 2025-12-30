// Generated macro for Error (enum)
macro_rules! Depcrate_file_includes_typesError {
() => {
// Module: crate::file::includes::types
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned when following includes."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to copy configuration file into buffer")] CopyBuffer (# [source] std :: io :: Error) , # [error ("Could not read included configuration file at '{}'" , path . display ())] Io { path : PathBuf , source : std :: io :: Error } , # [error (transparent)] Parse (# [from] parse :: Error) , # [error (transparent)] Interpolate (# [from] interpolate :: Error) , # [error ("The maximum allowed length {} of the file include chain built by following nested resolve_includes is exceeded" , . max_depth)] IncludeDepthExceeded { max_depth : u8 } , # [error ("Include paths from environment variables must not be relative as no config file paths exists as root")] MissingConfigPath , # [error ("The git directory must be provided to support `gitdir:` conditional includes")] MissingGitDir , # [error (transparent)] Realpath (# [from] gix_path :: realpath :: Error) , }
};
}
