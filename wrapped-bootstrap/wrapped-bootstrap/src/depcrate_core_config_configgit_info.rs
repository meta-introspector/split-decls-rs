// Generated macro for git_info (function)
macro_rules! Depcrate_core_config_configgit_info {
() => {
// Module: crate::core::config::config
// Provides: {"git_info"}
// Dependencies: {}
pub fn git_info (exec_ctx : & ExecutionContext , omit_git_hash : bool , dir : & Path) -> GitInfo { GitInfo :: new (omit_git_hash , dir , exec_ctx) }
};
}
