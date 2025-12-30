// Generated macro for fix_windows_verbatim_for_gcc (function)
macro_rules! Depcratefix_windows_verbatim_for_gcc {
() => {
// Module: crate
// Provides: {"fix_windows_verbatim_for_gcc"}
// Dependencies: {}
# [cfg (not (windows))] pub fn fix_windows_verbatim_for_gcc (p : & Path) -> PathBuf { p . to_path_buf () }
};
}
