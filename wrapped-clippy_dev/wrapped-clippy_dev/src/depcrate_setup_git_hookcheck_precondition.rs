// Generated macro for check_precondition (function)
macro_rules! Depcrate_setup_git_hookcheck_precondition {
() => {
// Module: crate::setup::git_hook
// Provides: {"check_precondition"}
// Dependencies: {}
fn check_precondition (force_override : bool) -> bool { let git_path = Path :: new (REPO_GIT_DIR) ; if ! git_path . exists () || ! git_path . is_dir () { eprintln ! ("error: clippy_dev was unable to find the `.git` directory") ; return false ; } let path = Path :: new (HOOK_TARGET_FILE) ; if path . exists () { if force_override { return delete_git_hook_file (path) ; } eprintln ! ("error: there is already a pre-commit hook installed") ; println ! ("info: use the `--force-override` flag to override the existing hook") ; return false ; } true }
};
}
