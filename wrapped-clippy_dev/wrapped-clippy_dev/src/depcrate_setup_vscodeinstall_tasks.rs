// Generated macro for install_tasks (function)
macro_rules! Depcrate_setup_vscodeinstall_tasks {
() => {
// Module: crate::setup::vscode
// Provides: {"install_tasks"}
// Dependencies: {}
pub fn install_tasks (force_override : bool) { if ! check_install_precondition (force_override) { return ; } match fs :: copy (TASK_SOURCE_FILE , TASK_TARGET_FILE) { Ok (_) => { println ! ("info: the task file can be removed with `cargo dev remove vscode-tasks`") ; println ! ("vscode tasks successfully installed") ; } , Err (err) => eprintln ! ("error: unable to copy `{TASK_SOURCE_FILE}` to `{TASK_TARGET_FILE}` ({err})") , } }
};
}
