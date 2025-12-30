// Generated macro for Error (enum)
macro_rules! Depcrate_dirwalkError {
() => {
// Module: crate::dirwalk
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [dirwalk()](crate::Repository::dirwalk())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Walk (# [from] gix_dir :: walk :: Error) , # [error ("A working tree is required to perform a directory walk")] MissingWorkDir , # [error (transparent)] Excludes (# [from] config :: exclude_stack :: Error) , # [error (transparent)] Pathspec (# [from] crate :: pathspec :: init :: Error) , # [error (transparent)] Prefix (# [from] gix_path :: realpath :: Error) , # [error (transparent)] FilesystemOptions (# [from] config :: boolean :: Error) , # [error ("Could not list worktrees to assure they are no candidates for deletion")] ListWorktrees (# [from] std :: io :: Error) , }
};
}
