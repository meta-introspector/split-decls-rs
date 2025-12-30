// Generated macro for DeleteOld (struct)
macro_rules! Depcrate_errorsDeleteOld {
() => {
// Module: crate::errors
// Provides: {"DeleteOld"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (incremental_delete_old)] pub (crate) struct DeleteOld < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
};
}
