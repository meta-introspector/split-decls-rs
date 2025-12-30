// Generated macro for macro_9 (macro)
macro_rules! Depcratemacro_9 {
() => {
// Module: crate
// Provides: {"macro_9"}
// Dependencies: {}
# [cfg (portable_atomic_disable_fiq)] # [cfg (not (portable_atomic_unsafe_assume_single_core))] compile_error ! ("`portable_atomic_disable_fiq` cfg (`disable-fiq` feature) may only be used together with `portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature)") ;
};
}
