// Generated macro for Error (enum)
macro_rules! Depcrate_statusError {
() => {
// Module: crate::status
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [status()](Repository::status)."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] DirwalkOptions (# [from] config :: boolean :: Error) , # [error (transparent)] ConfigureUntrackedFiles (# [from] config :: key :: GenericErrorWithValue) , }
};
}
