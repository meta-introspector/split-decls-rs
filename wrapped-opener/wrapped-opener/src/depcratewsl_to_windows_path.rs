// Generated macro for wsl_to_windows_path (function)
macro_rules! Depcratewsl_to_windows_path {
() => {
// Module: crate
// Provides: {"wsl_to_windows_path"}
// Dependencies: {}
# [cfg (not (target_os = "linux"))] fn wsl_to_windows_path (_path : & OsStr) -> Option < OsString > { unreachable ! () }
};
}
