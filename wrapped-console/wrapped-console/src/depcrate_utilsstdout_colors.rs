// Generated macro for STDOUT_COLORS (static)
macro_rules! Depcrate_utilsSTDOUT_COLORS {
() => {
// Module: crate::utils
// Provides: {"STDOUT_COLORS"}
// Dependencies: {}
static STDOUT_COLORS : Lazy < AtomicBool > = Lazy :: new (| | AtomicBool :: new (default_colors_enabled (& Term :: stdout ()))) ;
};
}
