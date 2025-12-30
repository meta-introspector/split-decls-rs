// Generated macro for dylib_path_var (function)
macro_rules! Depcrate_utils_shared_helpersdylib_path_var {
() => {
// Module: crate::utils::shared_helpers
// Provides: {"dylib_path_var"}
// Dependencies: {}
# [doc = " Returns the environment variable which the dynamic library lookup path"] # [doc = " resides in for this platform."] pub fn dylib_path_var () -> & 'static str { if cfg ! (any (target_os = "windows" , target_os = "cygwin")) { "PATH" } else if cfg ! (target_vendor = "apple") { "DYLD_LIBRARY_PATH" } else if cfg ! (target_os = "haiku") { "LIBRARY_PATH" } else if cfg ! (target_os = "aix") { "LIBPATH" } else { "LD_LIBRARY_PATH" } }
};
}
