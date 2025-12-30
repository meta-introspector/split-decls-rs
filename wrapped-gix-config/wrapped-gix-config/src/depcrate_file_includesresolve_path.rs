// Generated macro for resolve_path (function)
macro_rules! Depcrate_file_includesresolve_path {
() => {
// Module: crate::file::includes
// Provides: {"resolve_path"}
// Dependencies: {}
fn resolve_path (path : crate :: Path < '_ > , target_config_path : Option < & Path > , includes :: Options { interpolate : context , err_on_interpolation_failure , err_on_missing_config_path , .. } : includes :: Options < '_ > ,) -> Result < Option < PathBuf > , Error > { let path = match check_interpolation_result (err_on_interpolation_failure , path . interpolate (context)) ? { Some (p) => p , None => return Ok (None) , } ; let path : PathBuf = if path . is_relative () { if ! err_on_missing_config_path && target_config_path . is_none () { return Ok (None) ; } target_config_path . ok_or (Error :: MissingConfigPath) ? . parent () . expect ("path is a config file which naturally lives in a directory") . join (path) } else { path . into () } ; Ok (Some (path)) }
};
}
