// Generated macro for home_dir_inner (function)
macro_rules! Depcrate_windowshome_dir_inner {
() => {
// Module: crate::windows
// Provides: {"home_dir_inner"}
// Dependencies: {}
pub fn home_dir_inner () -> Option < PathBuf > { env :: var_os ("USERPROFILE") . filter (| s | ! s . is_empty ()) . map (PathBuf :: from) . or_else (home_dir_crt) }
};
}
