// Generated macro for get_lib_extension (function)
macro_rules! Depcrate_pathsget_lib_extension {
() => {
// Module: crate::paths
// Provides: {"get_lib_extension"}
// Dependencies: {}
# [doc = " See [`get_lib_filename`] for more details"] pub fn get_lib_extension (kind : & str) -> & str { match kind { "lib" | "rlib" => "rlib" , "staticlib" => { if cfg ! (windows) { "lib" } else { "a" } } "dylib" | "proc-macro" => { if cfg ! (windows) { "dll" } else if cfg ! (target_os = "macos") { "dylib" } else { "so" } } _ => unreachable ! () , } }
};
}
