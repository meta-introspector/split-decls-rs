// Generated macro for ensure_file_with_lock_free_access (function)
macro_rules! Depcrate_dylibensure_file_with_lock_free_access {
() => {
// Module: crate::dylib
// Provides: {"ensure_file_with_lock_free_access"}
// Dependencies: {}
# [cfg (unix)] fn ensure_file_with_lock_free_access (_temp_dir : & TempDir , path : & Utf8Path ,) -> io :: Result < Utf8PathBuf > { Ok (path . to_owned ()) }
};
}
