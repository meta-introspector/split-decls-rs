// Generated macro for remove_hook (function)
macro_rules! Depcrate_setup_git_hookremove_hook {
() => {
// Module: crate::setup::git_hook
// Provides: {"remove_hook"}
// Dependencies: {}
pub fn remove_hook () { let path = Path :: new (HOOK_TARGET_FILE) ; if path . exists () { if delete_git_hook_file (path) { println ! ("git hook successfully removed") ; } } else { println ! ("no pre-commit hook was found") ; } }
};
}
