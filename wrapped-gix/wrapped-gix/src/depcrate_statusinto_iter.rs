// Generated macro for into_iter (module)
macro_rules! Depcrate_statusinto_iter {
() => {
// Module: crate::status
// Provides: {"into_iter"}
// Dependencies: {}
# [doc = ""] pub mod into_iter { # [doc = " The error returned by [status::Platform::into_iter()](crate::status::Platform::into_iter())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Index (# [from] crate :: worktree :: open_index :: Error) , # [error ("Failed to spawn producer thread")] # [cfg (feature = "parallel")] SpawnThread (# [source] std :: io :: Error) , # [error (transparent)] # [cfg (not (feature = "parallel"))] IndexWorktreeStatus (# [from] crate :: status :: index_worktree :: Error) , # [error (transparent)] ConfigSkipHash (# [from] crate :: config :: boolean :: Error) , # [error (transparent)] PrepareSubmodules (# [from] crate :: submodule :: modules :: Error) , # [error ("Could not create an index for the head tree to compare with the worktree index")] HeadTreeIndex (# [from] crate :: repository :: index_from_tree :: Error) , # [error ("Could not obtain the tree id pointed to by `HEAD`")] HeadTreeId (# [from] crate :: reference :: head_tree_id :: Error) , # [error (transparent)] AttributesAndExcludes (# [from] crate :: repository :: attributes :: Error) , # [error (transparent)] Pathspec (# [from] crate :: pathspec :: init :: Error) , # [error (transparent)] HeadTreeDiff (# [from] crate :: status :: tree_index :: Error) , } }
};
}
