// Generated macro for start_lse128 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64start_lse128 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"start_lse128"}
// Dependencies: {}
# [cfg (not (portable_atomic_pre_llvm_16))] # [cfg (any (target_feature = "lse128" , portable_atomic_target_feature = "lse128" , all (not (portable_atomic_no_outline_atomics) , not (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2")) ,) ,))] macro_rules ! start_lse128 { () => { ".arch_extension lse128" } ; }
};
}
