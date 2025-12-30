// Generated macro for macro_386 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_386 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_386"}
// Dependencies: {}
# [cfg (not (any (target_feature = "lse128" , portable_atomic_target_feature = "lse128")))] atomic_rmw_ll_sc_3 ! { _atomic_or_ldxp_stxp as atomic_or (preserves_flags) , "orr {new_lo}, {prev_lo}, {val_lo}" , "orr {new_hi}, {prev_hi}, {val_hi}" , }
};
}
