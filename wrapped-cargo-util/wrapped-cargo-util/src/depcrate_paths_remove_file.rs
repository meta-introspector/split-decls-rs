// Generated macro for _remove_file (function)
macro_rules! Depcrate_paths_remove_file {
() => {
// Module: crate::paths
// Provides: {"_remove_file"}
// Dependencies: {}
fn _remove_file (p : & Path) -> Result < () > { # [cfg (target_os = "windows")] { use std :: os :: windows :: fs :: FileTypeExt ; let metadata = symlink_metadata (p) ? ; let file_type = metadata . file_type () ; if file_type . is_symlink_dir () { return remove_symlink_dir_with_permission_check (p) ; } } remove_file_with_permission_check (p) }
};
}
