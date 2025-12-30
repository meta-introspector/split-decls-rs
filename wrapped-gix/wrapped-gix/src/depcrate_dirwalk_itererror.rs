// Generated macro for Error (enum)
macro_rules! Depcrate_dirwalk_iterError {
() => {
// Module: crate::dirwalk::iter
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [Repository::dirwalk_iter()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to spawn producer thread")] # [cfg (feature = "parallel")] SpawnThread (# [from] std :: io :: Error) , # [error (transparent)] # [cfg (not (feature = "parallel"))] Dirwalk (# [from] dirwalk :: Error) , # [error (transparent)] # [cfg (not (feature = "parallel"))] DetachPathSpec (# [from] std :: io :: Error) , }
};
}
