// Generated macro for macro_389 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_389 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_389"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { _atomic_xor_ldxp_stxp as atomic_xor (preserves_flags) , "eor {new_lo}, {prev_lo}, {val_lo}" , "eor {new_hi}, {prev_hi}, {val_hi}" , }
};
}
