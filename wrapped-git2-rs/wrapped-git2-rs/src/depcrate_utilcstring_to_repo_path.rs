// Generated macro for cstring_to_repo_path (function)
macro_rules! Depcrate_utilcstring_to_repo_path {
() => {
// Module: crate::util
// Provides: {"cstring_to_repo_path"}
// Dependencies: {}
pub fn cstring_to_repo_path < T : IntoCString > (path : T) -> Result < CString , Error > { fixup_windows_path (path . into_c_string () ?) }
};
}
