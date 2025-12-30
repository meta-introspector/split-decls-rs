// Generated macro for get_lib_prefix (function)
macro_rules! Depcrate_pathsget_lib_prefix {
() => {
// Module: crate::paths
// Provides: {"get_lib_prefix"}
// Dependencies: {}
# [doc = " See [`get_lib_filename`] for more details"] pub fn get_lib_prefix (kind : & str) -> & str { match kind { "lib" | "rlib" => "lib" , "staticlib" | "dylib" | "proc-macro" => { if cfg ! (windows) { "" } else { "lib" } } _ => unreachable ! () , } }
};
}
