// Generated macro for Error (enum)
macro_rules! Depcrate_status_index_worktreeError {
() => {
// Module: crate::status::index_worktree
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [Repository::index_worktree_status()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("A working tree is required to perform a directory walk")] MissingWorkDir , # [error (transparent)] AttributesAndExcludes (# [from] crate :: repository :: attributes :: Error) , # [error (transparent)] Pathspec (# [from] crate :: pathspec :: init :: Error) , # [error (transparent)] Prefix (# [from] gix_path :: realpath :: Error) , # [error (transparent)] FilesystemOptions (# [from] config :: boolean :: Error) , # [error (transparent)] IndexAsWorktreeWithRenames (# [from] gix_status :: index_as_worktree_with_renames :: Error) , # [error (transparent)] StatOptions (# [from] config :: stat_options :: Error) , # [error (transparent)] ResourceCache (# [from] crate :: diff :: resource_cache :: Error) , }
};
}
