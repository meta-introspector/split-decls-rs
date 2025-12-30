// Generated macro for Error (enum)
macro_rules! Depcrate_tree_with_rewritesError {
() => {
// Module: crate::tree_with_rewrites
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`tree_with_rewrites()`](super::tree_with_rewrites())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Diff (# [from] crate :: tree :: Error) , # [error ("The user-provided callback failed")] ForEach (# [source] Box < dyn std :: error :: Error + Send + Sync + 'static >) , # [error ("Failure during rename tracking")] RenameTracking (# [from] crate :: rewrites :: tracker :: emit :: Error) , }
};
}
