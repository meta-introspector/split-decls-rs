// Generated macro for macro_387 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_387 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_387"}
// Dependencies: {}
# [cfg (not (any (target_feature = "lse128" , portable_atomic_target_feature = "lse128")))] atomic_rmw_cas_3 ! { _atomic_or_casp as atomic_or , "orr x4, x6, {val_lo}" , "orr x5, x7, {val_hi}" , }
};
}
