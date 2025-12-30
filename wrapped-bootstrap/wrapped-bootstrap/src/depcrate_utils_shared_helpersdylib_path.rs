// Generated macro for dylib_path (function)
macro_rules! Depcrate_utils_shared_helpersdylib_path {
() => {
// Module: crate::utils::shared_helpers
// Provides: {"dylib_path"}
// Dependencies: {}
# [doc = " Parses the `dylib_path_var()` environment variable, returning a list of"] # [doc = " paths that are members of this lookup path."] pub fn dylib_path () -> Vec < std :: path :: PathBuf > { let var = match std :: env :: var_os (dylib_path_var ()) { Some (v) => v , None => return vec ! [] , } ; std :: env :: split_paths (& var) . collect () }
};
}
