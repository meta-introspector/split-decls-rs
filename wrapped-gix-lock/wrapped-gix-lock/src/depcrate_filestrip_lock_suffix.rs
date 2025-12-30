// Generated macro for strip_lock_suffix (function)
macro_rules! Depcrate_filestrip_lock_suffix {
() => {
// Module: crate::file
// Provides: {"strip_lock_suffix"}
// Dependencies: {}
fn strip_lock_suffix (lock_path : & Path) -> PathBuf { let ext = lock_path . extension () . expect ("at least our own extension") . to_str () . expect ("no illegal UTF8 in extension") ; lock_path . with_extension (ext . split_at (ext . len () . saturating_sub (DOT_LOCK_SUFFIX . len ())) . 0) }
};
}
