// Generated macro for SessionGcFailed (struct)
macro_rules! Depcrate_errorsSessionGcFailed {
() => {
// Module: crate::errors
// Provides: {"SessionGcFailed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (incremental_session_gc_failed)] pub (crate) struct SessionGcFailed < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
};
}
