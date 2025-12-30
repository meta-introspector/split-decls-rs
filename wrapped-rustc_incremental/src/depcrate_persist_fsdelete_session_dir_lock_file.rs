// Generated macro for delete_session_dir_lock_file (function)
macro_rules! Depcrate_persist_fsdelete_session_dir_lock_file {
() => {
// Module: crate::persist::fs
// Provides: {"delete_session_dir_lock_file"}
// Dependencies: {}
fn delete_session_dir_lock_file (sess : & Session , lock_file_path : & Path) { if let Err (err) = safe_remove_file (lock_file_path) { sess . dcx () . emit_warn (errors :: DeleteLock { path : lock_file_path , err }) ; } }
};
}
