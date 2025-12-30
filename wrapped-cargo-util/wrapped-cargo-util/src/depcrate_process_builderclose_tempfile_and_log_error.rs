// Generated macro for close_tempfile_and_log_error (function)
macro_rules! Depcrate_process_builderclose_tempfile_and_log_error {
() => {
// Module: crate::process_builder
// Provides: {"close_tempfile_and_log_error"}
// Dependencies: {}
fn close_tempfile_and_log_error (file : NamedTempFile) { file . close () . unwrap_or_else (| e | { tracing :: warn ! ("failed to close temporary file: {e}") ; }) ; }
};
}
