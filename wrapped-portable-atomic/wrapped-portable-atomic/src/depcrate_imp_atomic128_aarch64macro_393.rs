// Generated macro for macro_393 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_393 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_393"}
// Dependencies: {}
atomic_rmw_ll_sc_2 ! { _atomic_neg_ldxp_stxp as atomic_neg , select_le_or_be ! ("negs {new_lo}, {prev_lo}" , "negs {new_hi}, {prev_hi}") , select_le_or_be ! ("ngc {new_hi}, {prev_hi}" , "ngc {new_lo}, {prev_lo}") , }
};
}
