// Generated macro for STDERR_TRUE_COLORS (static)
macro_rules! Depcrate_utilsSTDERR_TRUE_COLORS {
() => {
// Module: crate::utils
// Provides: {"STDERR_TRUE_COLORS"}
// Dependencies: {}
static STDERR_TRUE_COLORS : Lazy < AtomicBool > = Lazy :: new (| | AtomicBool :: new (default_true_colors_enabled (& Term :: stderr ()))) ;
};
}
