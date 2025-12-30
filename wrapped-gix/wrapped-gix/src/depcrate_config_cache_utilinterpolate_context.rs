// Generated macro for interpolate_context (function)
macro_rules! Depcrate_config_cache_utilinterpolate_context {
() => {
// Module: crate::config::cache::util
// Provides: {"interpolate_context"}
// Dependencies: {}
pub (crate) fn interpolate_context < 'a > (git_install_dir : Option < & 'a std :: path :: Path > , home_dir : Option < & 'a std :: path :: Path > ,) -> gix_config :: path :: interpolate :: Context < 'a > { gix_config :: path :: interpolate :: Context { git_install_dir , home_dir , home_for_user : Some (gix_config :: path :: interpolate :: home_for_user) , } }
};
}
