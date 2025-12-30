// Generated macro for macro_380 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_380 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_380"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_sub_casp as atomic_sub , select_le_or_be ! ("subs x4, x6, {val_lo}" , "subs x5, x7, {val_hi}") , select_le_or_be ! ("sbc x5, x7, {val_hi}" , "sbc x4, x6, {val_lo}") , }
};
}
