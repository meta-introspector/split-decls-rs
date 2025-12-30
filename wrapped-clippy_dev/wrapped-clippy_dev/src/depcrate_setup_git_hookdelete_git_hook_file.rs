// Generated macro for delete_git_hook_file (function)
macro_rules! Depcrate_setup_git_hookdelete_git_hook_file {
() => {
// Module: crate::setup::git_hook
// Provides: {"delete_git_hook_file"}
// Dependencies: {}
fn delete_git_hook_file (path : & Path) -> bool { if let Err (err) = fs :: remove_file (path) { eprintln ! ("error: unable to delete existing pre-commit git hook ({err})") ; false } else { true } }
};
}
