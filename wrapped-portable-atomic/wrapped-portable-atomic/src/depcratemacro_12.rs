// Generated macro for macro_12 (macro)
macro_rules! Depcratemacro_12 {
() => {
// Module: crate
// Provides: {"macro_12"}
// Dependencies: {}
# [cfg (all (portable_atomic_unsafe_assume_single_core , feature = "critical-section"))] compile_error ! ("you may not enable `critical-section` feature and `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature) at the same time") ;
};
}
