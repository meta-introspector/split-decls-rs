// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Notify error type."] # [doc = ""] # [doc = " Errors are emitted either at creation time of a `Watcher`, or during the event stream. They"] # [doc = " range from kernel errors to filesystem errors to argument errors."] # [doc = ""] # [doc = " Errors can be general, or they can be about specific paths or subtrees. In that later case, the"] # [doc = " error's `paths` field will be populated."] # [derive (Debug)] pub struct Error { # [doc = " Kind of the error."] pub kind : ErrorKind , # [doc = " Relevant paths to the error, if any."] pub paths : Vec < PathBuf > , }
};
}
