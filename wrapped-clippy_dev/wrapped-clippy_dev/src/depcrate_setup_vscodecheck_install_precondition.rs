// Generated macro for check_install_precondition (function)
macro_rules! Depcrate_setup_vscodecheck_install_precondition {
() => {
// Module: crate::setup::vscode
// Provides: {"check_install_precondition"}
// Dependencies: {}
fn check_install_precondition (force_override : bool) -> bool { let vs_dir_path = Path :: new (VSCODE_DIR) ; if vs_dir_path . exists () { if ! vs_dir_path . is_dir () { eprintln ! ("error: the `.vscode` path exists but seems to be a file") ; return false ; } let path = Path :: new (TASK_TARGET_FILE) ; if path . exists () { if force_override { return delete_vs_task_file (path) ; } eprintln ! ("error: there is already a `task.json` file inside the `{VSCODE_DIR}` directory") ; println ! ("info: use the `--force-override` flag to override the existing `task.json` file") ; return false ; } } else { match fs :: create_dir (vs_dir_path) { Ok (()) => { println ! ("info: created `{VSCODE_DIR}` directory for clippy") ; } , Err (err) => { eprintln ! ("error: the task target directory `{VSCODE_DIR}` could not be created ({err})") ; } , } } true }
};
}
