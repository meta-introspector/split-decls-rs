// Generated macro for add_dylib_path (function)
macro_rules! Depcrate_utiladd_dylib_path {
() => {
// Module: crate::util
// Provides: {"add_dylib_path"}
// Dependencies: {}
# [doc = " Adds a list of lookup paths to `cmd`'s dynamic library lookup path."] # [doc = " If the dylib_path_var is already set for this cmd, the old value will be overwritten!"] pub fn add_dylib_path (cmd : & mut Command , paths : impl Iterator < Item = impl Into < std :: path :: PathBuf > > ,) { let path_env = env :: var_os (dylib_env_var ()) ; let old_paths = path_env . as_ref () . map (env :: split_paths) ; let new_paths = paths . map (Into :: into) . chain (old_paths . into_iter () . flatten ()) ; cmd . env (dylib_env_var () , env :: join_paths (new_paths) . unwrap ()) ; }
};
}
