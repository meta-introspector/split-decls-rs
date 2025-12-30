// Generated macro for Error (enum)
macro_rules! Depcrate_index_as_worktree_typesError {
() => {
// Module: crate::index_as_worktree::types
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [index_as_worktree()`](crate::index_as_worktree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not convert path to UTF8")] IllformedUtf8 , # [error ("The clock was off when reading file related metadata after updating a file on disk")] Time (# [from] std :: time :: SystemTimeError) , # [error ("IO error while writing blob or reading file metadata or changing filetype")] Io (# [from] gix_hash :: io :: Error) , # [error ("Failed to obtain blob from object database")] Find (# [from] gix_object :: find :: existing_object :: Error) , # [error ("Could not determine status for submodule at '{rela_path}'")] SubmoduleStatus { rela_path : BString , source : Box < dyn std :: error :: Error + Send + Sync + 'static > , } , }
};
}
