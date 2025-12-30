// Generated macro for macro_11 (macro)
macro_rules! Depcratemacro_11 {
() => {
// Module: crate
// Provides: {"macro_11"}
// Dependencies: {}
# [cfg (portable_atomic_force_amo)] # [cfg (not (portable_atomic_unsafe_assume_single_core))] compile_error ! ("`portable_atomic_force_amo` cfg (`force-amo` feature) may only be used together with `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature)") ;
};
}
