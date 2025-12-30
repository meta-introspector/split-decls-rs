// Generated macro for home_dir (function)
macro_rules! Depcrate_config_cache_accesshome_dir {
() => {
// Module: crate::config::cache::access
// Provides: {"home_dir"}
// Dependencies: {}
pub (crate) fn home_dir (environment : crate :: open :: permissions :: Environment) -> Option < PathBuf > { gix_path :: env :: home_dir () . and_then (| path | environment . home . check_opt (path)) }
};
}
