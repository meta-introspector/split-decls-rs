// Generated macro for check_path_modifications_ (function)
macro_rules! Depcrate_core_config_configcheck_path_modifications_ {
() => {
// Module: crate::core::config::config
// Provides: {"check_path_modifications_"}
// Dependencies: {}
pub fn check_path_modifications_ < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , paths : & [& 'static str] ,) -> PathFreshness { let dwn_ctx = dwn_ctx . as_ref () ; dwn_ctx . path_modification_cache . lock () . unwrap () . entry (paths . to_vec ()) . or_insert_with (| | { check_path_modifications (dwn_ctx . src , & git_config (dwn_ctx . stage0_metadata) , paths , CiEnv :: current () ,) . unwrap () }) . clone () }
};
}
