// Generated macro for macro_396 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_396 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_396"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_max_casp as atomic_max , select_le_or_be ! ("cmp {val_lo}, x6" , "cmp {val_hi}, x7") , select_le_or_be ! ("sbcs xzr, {val_hi}, x7" , "sbcs xzr, {val_lo}, x6") , "csel x5, x7, {val_hi}, lt" , "csel x4, x6, {val_lo}, lt" , }
};
}
