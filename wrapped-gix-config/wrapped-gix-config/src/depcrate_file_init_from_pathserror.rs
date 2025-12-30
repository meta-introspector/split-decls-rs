// Generated macro for Error (enum)
macro_rules! Depcrate_file_init_from_pathsError {
() => {
// Module: crate::file::init::from_paths
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`File::from_paths_metadata()`] and [`File::from_path_no_includes()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The configuration file at \"{}\" could not be read" , path . display ())] Io { source : std :: io :: Error , path : std :: path :: PathBuf , } , # [error (transparent)] Init (# [from] init :: Error) , }
};
}
