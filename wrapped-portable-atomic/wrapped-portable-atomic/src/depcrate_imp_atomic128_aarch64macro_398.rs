// Generated macro for macro_398 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_398 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_398"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_umax_casp as atomic_umax , select_le_or_be ! ("cmp {val_lo}, x6" , "cmp {val_hi}, x7") , select_le_or_be ! ("sbcs xzr, {val_hi}, x7" , "sbcs xzr, {val_lo}, x6") , "csel x5, x7, {val_hi}, lo" , "csel x4, x6, {val_lo}, lo" , }
};
}
