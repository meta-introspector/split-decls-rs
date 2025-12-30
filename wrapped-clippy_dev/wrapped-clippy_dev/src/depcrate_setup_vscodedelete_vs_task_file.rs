// Generated macro for delete_vs_task_file (function)
macro_rules! Depcrate_setup_vscodedelete_vs_task_file {
() => {
// Module: crate::setup::vscode
// Provides: {"delete_vs_task_file"}
// Dependencies: {}
fn delete_vs_task_file (path : & Path) -> bool { if let Err (err) = fs :: remove_file (path) { eprintln ! ("error: unable to delete the existing `tasks.json` file ({err})") ; return false ; } true }
};
}
