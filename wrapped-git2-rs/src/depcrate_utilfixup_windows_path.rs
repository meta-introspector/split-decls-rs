// Generated macro for fixup_windows_path (function)
macro_rules! Depcrate_utilfixup_windows_path {
() => {
// Module: crate::util
// Provides: {"fixup_windows_path"}
// Dependencies: {}
# [cfg (not (windows))] fn fixup_windows_path (path : CString) -> Result < CString , Error > { Ok (path) }
};
}
