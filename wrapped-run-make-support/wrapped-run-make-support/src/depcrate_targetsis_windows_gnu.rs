// Generated macro for is_windows_gnu (function)
macro_rules! Depcrate_targetsis_windows_gnu {
() => {
// Module: crate::targets
// Provides: {"is_windows_gnu"}
// Dependencies: {}
# [doc = " Check if target is windows-gnu."] # [must_use] pub fn is_windows_gnu () -> bool { target () . ends_with ("windows-gnu") }
};
}
