// Generated macro for Error (struct)
macro_rules! Depcrate_errorsError {
() => {
// Module: crate::errors
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Contains an IO error that has a file path attached."] # [doc = ""] # [doc = " This type is never returned directly, but is instead wrapped inside yet"] # [doc = " another IO error."] # [derive (Debug)] pub (crate) struct Error { kind : ErrorKind , source : io :: Error , path : PathBuf , }
};
}
