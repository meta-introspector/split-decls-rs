macro_rules! Error {
    () => {
        # [doc = " The error returned by [index_as_worktree_with_renames()`](crate::index_as_worktree_with_renames())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] TrackedFileModifications (# [from] crate :: index_as_worktree :: Error) , # [error (transparent)] DirWalk (gix_dir :: walk :: Error) , # [error (transparent)] SpawnThread (std :: io :: Error) , # [error ("Failed to change the context for querying gitattributes to the respective path")] SetAttributeContext (std :: io :: Error) , # [error ("Could not open worktree file for reading")] OpenWorktreeFile (std :: io :: Error) , # [error (transparent)] HashFile (gix_hash :: io :: Error) , # [error ("Could not read worktree link content")] ReadLink (std :: io :: Error) , # [error (transparent)] ConvertToGit (# [from] gix_filter :: pipeline :: convert :: to_git :: Error) , # [error (transparent)] RewriteTracker (# [from] gix_diff :: rewrites :: tracker :: emit :: Error) , }
    };
}

Error!()