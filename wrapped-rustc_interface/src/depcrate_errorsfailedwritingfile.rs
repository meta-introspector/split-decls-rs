// Generated macro for FailedWritingFile (struct)
macro_rules! Depcrate_errorsFailedWritingFile {
() => {
// Module: crate::errors
// Provides: {"FailedWritingFile"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (interface_failed_writing_file)] pub struct FailedWritingFile < 'a > { pub path : & 'a Path , pub error : io :: Error , }
};
}
