// Generated macro for Error (enum)
macro_rules! Depcrate_indexError {
() => {
// Module: crate::index
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`index()`](crate::index())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Cannot diff indices that contain sparse entries")] IsSparse , # [error ("Unmerged entries aren't allowed in the left-hand index, only in the right-hand index")] LhsHasUnmerged , # [error ("The callback indicated failure")] Callback (# [source] Box < dyn std :: error :: Error + Send + Sync >) , # [error ("Failure during rename tracking")] RenameTracking (# [from] crate :: rewrites :: tracker :: emit :: Error) , }
};
}
