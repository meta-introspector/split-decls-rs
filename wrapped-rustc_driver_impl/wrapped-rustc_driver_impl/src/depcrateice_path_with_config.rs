// Generated macro for ice_path_with_config (function)
macro_rules! Depcrateice_path_with_config {
() => {
// Module: crate
// Provides: {"ice_path_with_config"}
// Dependencies: {}
fn ice_path_with_config (config : Option < & UnstableOptions >) -> & 'static Option < PathBuf > { if ICE_PATH . get () . is_some () && config . is_some () && cfg ! (debug_assertions) { tracing :: warn ! ("ICE_PATH has already been initialized -- files may be emitted at unintended paths") } ICE_PATH . get_or_init (| | { if ! rustc_feature :: UnstableFeatures :: from_environment (None) . is_nightly_build () { return None ; } let mut path = match std :: env :: var_os ("RUSTC_ICE") { Some (s) => { if s == "0" { return None ; } if let Some (unstable_opts) = config && unstable_opts . metrics_dir . is_some () { tracing :: warn ! ("ignoring -Zerror-metrics in favor of RUSTC_ICE for destination of ICE report files") ; } PathBuf :: from (s) } None => config . and_then (| unstable_opts | unstable_opts . metrics_dir . to_owned ()) . or_else (| | std :: env :: current_dir () . ok ()) . unwrap_or_default () , } ; let file_now = jiff :: Zoned :: now () . strftime ("%Y-%m-%dT%H_%M_%S") ; let pid = std :: process :: id () ; path . push (format ! ("rustc-ice-{file_now}-{pid}.txt")) ; Some (path) }) }
};
}
