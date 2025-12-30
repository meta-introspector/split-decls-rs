// Generated macro for Error (enum)
macro_rules! Depcrate_helperError {
() => {
// Module: crate::helper
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in the [credentials helper invocation][crate::helper::invoke()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ContextDecode (# [from] protocol :: context :: decode :: Error) , # [error ("An IO error occurred while communicating to the credentials helper")] Io (# [from] std :: io :: Error) , # [error (transparent)] CredentialsHelperFailed { source : std :: io :: Error } , }
};
}
