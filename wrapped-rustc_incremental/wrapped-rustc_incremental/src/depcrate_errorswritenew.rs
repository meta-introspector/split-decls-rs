// Generated macro for WriteNew (struct)
macro_rules! Depcrate_errorsWriteNew {
() => {
// Module: crate::errors
// Provides: {"WriteNew"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (incremental_write_new)] pub (crate) struct WriteNew < 'a > { pub name : & 'a str , pub path : PathBuf , pub err : std :: io :: Error , }
};
}
