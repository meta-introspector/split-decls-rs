// Generated macro for remove (function)
macro_rules! Depcrate_core_downloadremove {
() => {
// Module: crate::core::download
// Provides: {"remove"}
// Dependencies: {}
pub (crate) fn remove (exec_ctx : & ExecutionContext , f : & Path) { if exec_ctx . dry_run () { return ; } fs :: remove_file (f) . unwrap_or_else (| _ | panic ! ("failed to remove {f:?}")) ; }
};
}
