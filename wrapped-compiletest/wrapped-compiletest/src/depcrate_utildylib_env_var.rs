// Generated macro for dylib_env_var (function)
macro_rules! Depcrate_utildylib_env_var {
() => {
// Module: crate::util
// Provides: {"dylib_env_var"}
// Dependencies: {}
# [doc = " The name of the environment variable that holds dynamic library locations."] pub fn dylib_env_var () -> & 'static str { if cfg ! (any (windows , target_os = "cygwin")) { "PATH" } else if cfg ! (target_vendor = "apple") { "DYLD_LIBRARY_PATH" } else if cfg ! (target_os = "haiku") { "LIBRARY_PATH" } else if cfg ! (target_os = "aix") { "LIBPATH" } else { "LD_LIBRARY_PATH" } }
};
}
