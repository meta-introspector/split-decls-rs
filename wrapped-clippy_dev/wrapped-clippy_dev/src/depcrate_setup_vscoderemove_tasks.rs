// Generated macro for remove_tasks (function)
macro_rules! Depcrate_setup_vscoderemove_tasks {
() => {
// Module: crate::setup::vscode
// Provides: {"remove_tasks"}
// Dependencies: {}
pub fn remove_tasks () { let path = Path :: new (TASK_TARGET_FILE) ; if path . exists () { if delete_vs_task_file (path) { try_delete_vs_directory_if_empty () ; println ! ("vscode tasks successfully removed") ; } } else { println ! ("no vscode tasks were found") ; } }
};
}
