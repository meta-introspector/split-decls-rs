// Generated macro for use_368 (use)
macro_rules! Depcrate_imp_atomic128_aarch64use_368 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"use_368"}
// Dependencies: {}
# [cfg (not (any (target_feature = "lse128" , portable_atomic_target_feature = "lse128")))] # [cfg (not (all (any (target_feature = "lse" , portable_atomic_target_feature = "lse") , not (portable_atomic_ll_sc_rmw) ,)))] use self :: _atomic_swap_ldxp_stxp as atomic_swap ;
};
}
