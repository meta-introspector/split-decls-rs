// Generated macro for mtime (function)
macro_rules! Depcrate_servemtime {
() => {
// Module: crate::serve
// Provides: {"mtime"}
// Dependencies: {}
fn mtime (path : & str) -> SystemTime { log_err_and_continue (fs :: metadata (path) , path . as_ref ()) . and_then (| metadata | log_err_and_continue (metadata . modified () , path . as_ref ())) . unwrap_or (SystemTime :: UNIX_EPOCH) }
};
}
