// Generated macro for dylib_path_envvar (function)
macro_rules! Depcrate_pathsdylib_path_envvar {
() => {
// Module: crate::paths
// Provides: {"dylib_path_envvar"}
// Dependencies: {}
# [doc = " Returns the name of the environment variable used for searching for"] # [doc = " dynamic libraries."] pub fn dylib_path_envvar () -> & 'static str { if cfg ! (windows) { "PATH" } else if cfg ! (target_os = "macos") { "DYLD_FALLBACK_LIBRARY_PATH" } else if cfg ! (target_os = "aix") { "LIBPATH" } else { "LD_LIBRARY_PATH" } }
};
}
