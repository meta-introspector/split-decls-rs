// Generated macro for install_hook (function)
macro_rules! Depcrate_setup_git_hookinstall_hook {
() => {
// Module: crate::setup::git_hook
// Provides: {"install_hook"}
// Dependencies: {}
pub fn install_hook (force_override : bool) { if ! check_precondition (force_override) { return ; } match fs :: copy (HOOK_SOURCE_FILE , HOOK_TARGET_FILE) { Ok (_) => { println ! ("info: the hook can be removed with `cargo dev remove git-hook`") ; println ! ("git hook successfully installed") ; } , Err (err) => eprintln ! ("error: unable to copy `{HOOK_SOURCE_FILE}` to `{HOOK_TARGET_FILE}` ({err})") , } }
};
}
