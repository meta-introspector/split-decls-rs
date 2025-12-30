// Generated macro for is_session_directory_lock_file (function)
macro_rules! Depcrate_persist_fsis_session_directory_lock_file {
() => {
// Module: crate::persist::fs
// Provides: {"is_session_directory_lock_file"}
// Dependencies: {}
fn is_session_directory_lock_file (file_name : & str) -> bool { file_name . starts_with ("s-") && file_name . ends_with (LOCK_FILE_EXT) }
};
}
