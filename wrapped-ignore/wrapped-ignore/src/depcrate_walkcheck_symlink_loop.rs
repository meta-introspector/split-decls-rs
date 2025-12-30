// Generated macro for check_symlink_loop (function)
macro_rules! Depcrate_walkcheck_symlink_loop {
() => {
// Module: crate::walk
// Provides: {"check_symlink_loop"}
// Dependencies: {}
fn check_symlink_loop (ig_parent : & Ignore , child_path : & Path , child_depth : usize ,) -> Result < () , Error > { let hchild = Handle :: from_path (child_path) . map_err (| err | { Error :: from (err) . with_path (child_path) . with_depth (child_depth) }) ? ; for ig in ig_parent . parents () . take_while (| ig | ! ig . is_absolute_parent ()) { let h = Handle :: from_path (ig . path ()) . map_err (| err | { Error :: from (err) . with_path (child_path) . with_depth (child_depth) }) ? ; if hchild == h { return Err (Error :: Loop { ancestor : ig . path () . to_path_buf () , child : child_path . to_path_buf () , } . with_depth (child_depth)) ; } } Ok (()) }
};
}
