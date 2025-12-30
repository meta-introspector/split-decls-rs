// Generated macro for TransitProcessResult (enum)
macro_rules! Depcrate_dirTransitProcessResult {
() => {
// Module: crate::dir
// Provides: {"TransitProcessResult"}
// Dependencies: {}
# [doc = " Available returns codes for user decide"] pub enum TransitProcessResult { # [doc = " Rewrite exist file or directory."] Overwrite , # [doc = " Rewrite for all exist files or directories."] OverwriteAll , # [doc = " Skip current problem file or directory."] Skip , # [doc = " Skip for all problems file or directory."] SkipAll , # [doc = " Retry current operation."] Retry , # [doc = " Abort current operation."] Abort , # [doc = " Continue execute process if process not have error and abort if process content error."] ContinueOrAbort , }
};
}
