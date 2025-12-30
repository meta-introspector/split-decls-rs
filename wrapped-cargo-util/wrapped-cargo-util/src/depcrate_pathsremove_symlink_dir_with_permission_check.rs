// Generated macro for remove_symlink_dir_with_permission_check (function)
macro_rules! Depcrate_pathsremove_symlink_dir_with_permission_check {
() => {
// Module: crate::paths
// Provides: {"remove_symlink_dir_with_permission_check"}
// Dependencies: {}
# [cfg (target_os = "windows")] fn remove_symlink_dir_with_permission_check (p : & Path) -> Result < () > { remove_with_permission_check (fs :: remove_dir , p) . with_context (| | format ! ("failed to remove symlink dir `{}`" , p . display ())) }
};
}
