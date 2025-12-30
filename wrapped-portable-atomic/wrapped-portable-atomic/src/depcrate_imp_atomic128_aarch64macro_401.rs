// Generated macro for macro_401 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_401 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_401"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { _atomic_umin_ldxp_stxp as atomic_umin , select_le_or_be ! ("cmp {val_lo}, {prev_lo}" , "cmp {val_hi}, {prev_hi}") , select_le_or_be ! ("sbcs xzr, {val_hi}, {prev_hi}" , "sbcs xzr, {val_lo}, {prev_lo}") , "csel {new_hi}, {prev_hi}, {val_hi}, hs" , "csel {new_lo}, {prev_lo}, {val_lo}, hs" , }
};
}
