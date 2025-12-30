// Generated macro for Error (enum)
macro_rules! Depcrate_openError {
() => {
// Module: crate::open
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`crate::open()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to load the git configuration")] Config (# [from] config :: Error) , # [error ("\"{path}\" does not appear to be a git repository")] NotARepository { source : gix_discover :: is_git :: Error , path : PathBuf , } , # [error (transparent)] Io (# [from] std :: io :: Error) , # [error ("The git directory at '{}' is considered unsafe as it's not owned by the current user." , . path . display ())] UnsafeGitDir { path : PathBuf } , # [error (transparent)] EnvironmentAccessDenied (# [from] gix_sec :: permission :: Error < std :: path :: PathBuf >) , # [error (transparent)] PrefixNotRelative (# [from] gix_path :: relative_path :: Error) , }
};
}
