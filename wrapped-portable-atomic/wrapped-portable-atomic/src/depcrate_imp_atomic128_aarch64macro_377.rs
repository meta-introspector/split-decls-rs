// Generated macro for macro_377 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_377 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_377"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { _atomic_add_ldxp_stxp as atomic_add , select_le_or_be ! ("adds {new_lo}, {prev_lo}, {val_lo}" , "adds {new_hi}, {prev_hi}, {val_hi}") , select_le_or_be ! ("adc {new_hi}, {prev_hi}, {val_hi}" , "adc {new_lo}, {prev_lo}, {val_lo}") , }
};
}
