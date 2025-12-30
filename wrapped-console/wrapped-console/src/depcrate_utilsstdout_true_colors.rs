// Generated macro for STDOUT_TRUE_COLORS (static)
macro_rules! Depcrate_utilsSTDOUT_TRUE_COLORS {
() => {
// Module: crate::utils
// Provides: {"STDOUT_TRUE_COLORS"}
// Dependencies: {}
static STDOUT_TRUE_COLORS : Lazy < AtomicBool > = Lazy :: new (| | AtomicBool :: new (default_true_colors_enabled (& Term :: stdout ()))) ;
};
}
