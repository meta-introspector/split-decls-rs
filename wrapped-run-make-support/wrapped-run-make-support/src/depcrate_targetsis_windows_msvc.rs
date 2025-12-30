// Generated macro for is_windows_msvc (function)
macro_rules! Depcrate_targetsis_windows_msvc {
() => {
// Module: crate::targets
// Provides: {"is_windows_msvc"}
// Dependencies: {}
# [doc = " Check if target is windows-msvc."] # [must_use] pub fn is_windows_msvc () -> bool { target () . ends_with ("windows-msvc") }
};
}
