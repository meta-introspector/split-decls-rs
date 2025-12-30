// Generated macro for error (function)
macro_rules! Depcrate_outputerror {
() => {
// Module: crate::output
// Provides: {"error"}
// Dependencies: {}
# [doc = " The `error` instruction tells Cargo to display an error after the build script has finished"] # [doc = " running, and then fail the build."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " Build script libraries should carefully consider if they want to use [`error`] versus"] # [doc = " returning a `Result`. It may be better to return a `Result`, and allow the caller to decide if the"] # [doc = " error is fatal or not. The caller can then decide whether or not to display the `Err` variant"] # [doc = " using [`error`]."] # [doc = ""] # [doc = " </div>"] # [doc = respected_msrv ! ("1.84")] # [track_caller] pub fn error (message : & str) { if message . contains ('\n') { panic ! ("cannot emit error: message contains newline") ; } emit ("error" , message) ; }
};
}
