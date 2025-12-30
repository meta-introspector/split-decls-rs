// Generated macro for DeleteLock (struct)
macro_rules! Depcrate_errorsDeleteLock {
() => {
// Module: crate::errors
// Provides: {"DeleteLock"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (incremental_delete_lock)] pub (crate) struct DeleteLock < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
};
}
