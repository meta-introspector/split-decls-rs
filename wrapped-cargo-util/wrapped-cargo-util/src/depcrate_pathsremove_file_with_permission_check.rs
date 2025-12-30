// Generated macro for remove_file_with_permission_check (function)
macro_rules! Depcrate_pathsremove_file_with_permission_check {
() => {
// Module: crate::paths
// Provides: {"remove_file_with_permission_check"}
// Dependencies: {}
fn remove_file_with_permission_check (p : & Path) -> Result < () > { remove_with_permission_check (fs :: remove_file , p) . with_context (| | format ! ("failed to remove file `{}`" , p . display ())) }
};
}
