// Generated macro for is_session_directory (function)
macro_rules! Depcrate_persist_fsis_session_directory {
() => {
// Module: crate::persist::fs
// Provides: {"is_session_directory"}
// Dependencies: {}
fn is_session_directory (directory_name : & str) -> bool { directory_name . starts_with ("s-") && ! directory_name . ends_with (LOCK_FILE_EXT) }
};
}
