// Generated macro for Error (enum)
macro_rules! Depcrate_is_active_platformError {
() => {
// Module: crate::is_active_platform
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [File::names_and_active_state](crate::File::names_and_active_state())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] NormalizePattern (# [from] gix_pathspec :: normalize :: Error) , # [error (transparent)] ParsePattern (# [from] gix_pathspec :: parse :: Error) , }
};
}
