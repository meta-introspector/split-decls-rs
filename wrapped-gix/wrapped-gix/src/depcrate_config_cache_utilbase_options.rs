// Generated macro for base_options (function)
macro_rules! Depcrate_config_cache_utilbase_options {
() => {
// Module: crate::config::cache::util
// Provides: {"base_options"}
// Dependencies: {}
pub (crate) fn base_options (lossy : bool , lenient : bool) -> gix_config :: file :: init :: Options < 'static > { gix_config :: file :: init :: Options { lossy , ignore_io_errors : lenient , .. Default :: default () } }
};
}
