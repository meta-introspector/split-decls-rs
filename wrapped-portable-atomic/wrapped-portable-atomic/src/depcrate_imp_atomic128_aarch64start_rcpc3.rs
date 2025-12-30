// Generated macro for start_rcpc3 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64start_rcpc3 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"start_rcpc3"}
// Dependencies: {}
# [cfg (not (portable_atomic_pre_llvm_16))] # [cfg (any (target_feature = "rcpc3" , portable_atomic_target_feature = "rcpc3" , all (not (portable_atomic_no_outline_atomics) , not (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2")) ,) ,))] macro_rules ! start_rcpc3 { () => { ".arch_extension rcpc3" } ; }
};
}
