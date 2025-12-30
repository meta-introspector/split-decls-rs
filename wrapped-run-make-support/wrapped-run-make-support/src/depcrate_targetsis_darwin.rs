// Generated macro for is_darwin (function)
macro_rules! Depcrate_targetsis_darwin {
() => {
// Module: crate::targets
// Provides: {"is_darwin"}
// Dependencies: {}
# [doc = " Check if target uses macOS."] # [must_use] pub fn is_darwin () -> bool { target () . contains ("darwin") }
};
}
