// Generated macro for macro_400 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_400 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_400"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_min_casp as atomic_min , select_le_or_be ! ("cmp {val_lo}, x6" , "cmp {val_hi}, x7") , select_le_or_be ! ("sbcs xzr, {val_hi}, x7" , "sbcs xzr, {val_lo}, x6") , "csel x5, x7, {val_hi}, ge" , "csel x4, x6, {val_lo}, ge" , }
};
}
