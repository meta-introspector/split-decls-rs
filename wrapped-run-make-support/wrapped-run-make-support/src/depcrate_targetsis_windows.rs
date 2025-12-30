// Generated macro for is_windows (function)
macro_rules! Depcrate_targetsis_windows {
() => {
// Module: crate::targets
// Provides: {"is_windows"}
// Dependencies: {}
# [doc = " Check if target is windows-like."] # [must_use] pub fn is_windows () -> bool { target () . contains ("windows") }
};
}
