// Generated macro for STDERR_COLORS (static)
macro_rules! Depcrate_utilsSTDERR_COLORS {
() => {
// Module: crate::utils
// Provides: {"STDERR_COLORS"}
// Dependencies: {}
static STDERR_COLORS : Lazy < AtomicBool > = Lazy :: new (| | AtomicBool :: new (default_colors_enabled (& Term :: stderr ()))) ;
};
}
