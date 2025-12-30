// Generated macro for set_host_compiler_dylib_path (function)
macro_rules! Depcrate_utilset_host_compiler_dylib_path {
() => {
// Module: crate::util
// Provides: {"set_host_compiler_dylib_path"}
// Dependencies: {}
# [doc = " Set the runtime library paths as needed for running the host compilers (rustc/rustdoc/etc)."] pub (crate) fn set_host_compiler_dylib_path (cmd : & mut Command) { let ld_lib_path_envvar = env_var ("LD_LIB_PATH_ENVVAR") ; cmd . env (& ld_lib_path_envvar , { let mut paths = vec ! [] ; paths . push (cwd ()) ; paths . push (PathBuf :: from (env_var ("HOST_RUSTC_DYLIB_PATH"))) ; for p in std :: env :: split_paths (& env_var (& ld_lib_path_envvar)) { paths . push (p . to_path_buf ()) ; } std :: env :: join_paths (paths . iter ()) . unwrap () }) ; }
};
}
