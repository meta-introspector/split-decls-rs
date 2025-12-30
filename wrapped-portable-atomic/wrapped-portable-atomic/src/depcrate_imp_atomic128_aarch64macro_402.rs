// Generated macro for macro_402 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_402 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_402"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_umin_casp as atomic_umin , select_le_or_be ! ("cmp {val_lo}, x6" , "cmp {val_hi}, x7") , select_le_or_be ! ("sbcs xzr, {val_hi}, x7" , "sbcs xzr, {val_lo}, x6") , "csel x5, x7, {val_hi}, hs" , "csel x4, x6, {val_lo}, hs" , }
};
}
