// Generated macro for ErrorWritingDependencies (struct)
macro_rules! Depcrate_errorsErrorWritingDependencies {
() => {
// Module: crate::errors
// Provides: {"ErrorWritingDependencies"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (interface_error_writing_dependencies)] pub struct ErrorWritingDependencies < 'a > { pub path : & 'a Path , pub error : io :: Error , }
};
}
