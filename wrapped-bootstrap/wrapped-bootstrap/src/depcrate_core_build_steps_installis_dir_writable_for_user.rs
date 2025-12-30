// Generated macro for is_dir_writable_for_user (function)
macro_rules! Depcrate_core_build_steps_installis_dir_writable_for_user {
() => {
// Module: crate::core::build_steps::install
// Provides: {"is_dir_writable_for_user"}
// Dependencies: {}
fn is_dir_writable_for_user (dir : & Path) -> bool { let tmp = dir . join (".tmp") ; match fs :: create_dir_all (& tmp) { Ok (_) => { fs :: remove_dir_all (tmp) . unwrap () ; true } Err (e) => { if e . kind () == std :: io :: ErrorKind :: PermissionDenied { false } else { panic ! ("Failed the write access check for the current user. {e}") ; } } } }
};
}
