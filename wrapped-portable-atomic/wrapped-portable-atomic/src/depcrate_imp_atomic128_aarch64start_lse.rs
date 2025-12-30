// Generated macro for start_lse (macro)
macro_rules! Depcrate_imp_atomic128_aarch64start_lse {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"start_lse"}
// Dependencies: {}
# [cfg (any (target_feature = "lse" , portable_atomic_target_feature = "lse" , not (portable_atomic_no_outline_atomics) ,))] macro_rules ! start_lse { () => { ".arch_extension lse" } ; }
};
}
