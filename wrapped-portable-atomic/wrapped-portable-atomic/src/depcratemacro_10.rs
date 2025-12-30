// Generated macro for macro_10 (macro)
macro_rules! Depcratemacro_10 {
() => {
// Module: crate
// Provides: {"macro_10"}
// Dependencies: {}
# [cfg (portable_atomic_s_mode)] # [cfg (not (portable_atomic_unsafe_assume_single_core))] compile_error ! ("`portable_atomic_s_mode` cfg (`s-mode` feature) may only be used together with `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature)") ;
};
}
