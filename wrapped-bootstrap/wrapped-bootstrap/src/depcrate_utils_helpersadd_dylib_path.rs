// Generated macro for add_dylib_path (function)
macro_rules! Depcrate_utils_helpersadd_dylib_path {
() => {
// Module: crate::utils::helpers
// Provides: {"add_dylib_path"}
// Dependencies: {}
# [doc = " Adds a list of lookup paths to `cmd`'s dynamic library lookup path."] # [doc = " If the dylib_path_var is already set for this cmd, the old value will be overwritten!"] pub fn add_dylib_path (path : Vec < PathBuf > , cmd : & mut BootstrapCommand) { let mut list = dylib_path () ; for path in path { list . insert (0 , path) ; } cmd . env (dylib_path_var () , t ! (env :: join_paths (list))) ; }
};
}
