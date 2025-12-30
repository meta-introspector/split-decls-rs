// Generated macro for use_367 (use)
macro_rules! Depcrate_imp_atomic128_aarch64use_367 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"use_367"}
// Dependencies: {}
# [cfg (not (any (target_feature = "lse128" , portable_atomic_target_feature = "lse128")))] # [cfg (all (any (target_feature = "lse" , portable_atomic_target_feature = "lse") , not (portable_atomic_ll_sc_rmw) ,))] use self :: _atomic_swap_casp as atomic_swap ;
};
}
