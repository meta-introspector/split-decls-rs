// Generated macro for find_miri (function)
macro_rules! Depcrate_utilfind_miri {
() => {
// Module: crate::util
// Provides: {"find_miri"}
// Dependencies: {}
# [doc = " Returns the path to the `miri` binary"] pub fn find_miri () -> PathBuf { if let Some (path) = env :: var_os ("MIRI") { return path . into () ; } let mut path = std :: env :: current_exe () . expect ("current executable path invalid") ; path . set_file_name (format ! ("miri{}" , env :: consts :: EXE_SUFFIX)) ; path }
};
}
