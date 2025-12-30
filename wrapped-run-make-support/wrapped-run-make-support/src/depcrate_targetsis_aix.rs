// Generated macro for is_aix (function)
macro_rules! Depcrate_targetsis_aix {
() => {
// Module: crate::targets
// Provides: {"is_aix"}
// Dependencies: {}
# [doc = " Check if target uses AIX."] # [must_use] pub fn is_aix () -> bool { target () . contains ("aix") }
};
}
