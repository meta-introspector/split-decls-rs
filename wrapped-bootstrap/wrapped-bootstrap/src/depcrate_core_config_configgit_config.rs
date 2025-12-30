// Generated macro for git_config (function)
macro_rules! Depcrate_core_config_configgit_config {
() => {
// Module: crate::core::config::config
// Provides: {"git_config"}
// Dependencies: {}
pub fn git_config (stage0_metadata : & build_helper :: stage0_parser :: Stage0) -> GitConfig < '_ > { GitConfig { nightly_branch : & stage0_metadata . config . nightly_branch , git_merge_commit_email : & stage0_metadata . config . git_merge_commit_email , } }
};
}
