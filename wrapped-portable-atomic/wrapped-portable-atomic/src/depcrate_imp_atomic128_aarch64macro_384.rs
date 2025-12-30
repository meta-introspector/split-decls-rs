// Generated macro for macro_384 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_384 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_384"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { _atomic_nand_ldxp_stxp as atomic_nand (preserves_flags) , "and {new_lo}, {prev_lo}, {val_lo}" , "and {new_hi}, {prev_hi}, {val_hi}" , "mvn {new_lo}, {new_lo}" , "mvn {new_hi}, {new_hi}" , }
};
}
