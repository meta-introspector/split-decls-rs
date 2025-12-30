// Generated macro for macro_382 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_382 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_382"}
// Dependencies: {}
# [cfg (not (any (target_feature = "lse128" , portable_atomic_target_feature = "lse128")))] atomic_rmw_cas_3 ! { _atomic_and_casp as atomic_and , "and x4, x6, {val_lo}" , "and x5, x7, {val_hi}" , }
};
}
