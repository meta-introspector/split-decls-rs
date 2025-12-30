// Generated macro for has_changes_from_upstream (function)
macro_rules! Depcrate_core_config_confighas_changes_from_upstream {
() => {
// Module: crate::core::config::config
// Provides: {"has_changes_from_upstream"}
// Dependencies: {}
pub fn has_changes_from_upstream < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , paths : & [& 'static str] ,) -> bool { let dwn_ctx = dwn_ctx . as_ref () ; match check_path_modifications_ (dwn_ctx , paths) { PathFreshness :: LastModifiedUpstream { .. } => false , PathFreshness :: HasLocalModifications { .. } | PathFreshness :: MissingUpstream => true , } }
};
}
