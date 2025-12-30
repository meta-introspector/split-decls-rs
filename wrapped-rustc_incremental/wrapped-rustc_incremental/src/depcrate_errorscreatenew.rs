// Generated macro for CreateNew (struct)
macro_rules! Depcrate_errorsCreateNew {
() => {
// Module: crate::errors
// Provides: {"CreateNew"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (incremental_create_new)] pub (crate) struct CreateNew < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
};
}
