// Generated macro for macro_379 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_379 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_379"}
// Dependencies: {}
atomic_rmw_ll_sc_3 ! { _atomic_sub_ldxp_stxp as atomic_sub , select_le_or_be ! ("subs {new_lo}, {prev_lo}, {val_lo}" , "subs {new_hi}, {prev_hi}, {val_hi}") , select_le_or_be ! ("sbc {new_hi}, {prev_hi}, {val_hi}" , "sbc {new_lo}, {prev_lo}, {val_lo}") , }
};
}
