// Generated macro for compute_src_directory (function)
macro_rules! Depcrate_core_config_configcompute_src_directory {
() => {
// Module: crate::core::config::config
// Provides: {"compute_src_directory"}
// Dependencies: {}
fn compute_src_directory (src_dir : Option < PathBuf > , exec_ctx : & ExecutionContext) -> Option < PathBuf > { if let Some (src) = src_dir { return Some (src) ; } else { let mut cmd = helpers :: git (None) ; cmd . arg ("rev-parse") . arg ("--show-cdup") ; let output = cmd . allow_failure () . run_capture_stdout (exec_ctx) ; if output . is_success () { let git_root_relative = output . stdout () ; let git_root = env :: current_dir () . unwrap () . join (PathBuf :: from (git_root_relative . trim ())) . canonicalize () . unwrap () ; let s = git_root . to_str () . unwrap () ; let git_root = match s . strip_prefix ("\\\\?\\") { Some (p) => PathBuf :: from (p) , None => git_root , } ; if git_root . join ("src") . join ("stage0") . exists () { return Some (git_root) ; } } else { } } ; None }
};
}
