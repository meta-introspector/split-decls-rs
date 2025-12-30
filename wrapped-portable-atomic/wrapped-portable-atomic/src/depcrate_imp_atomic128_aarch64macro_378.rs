// Generated macro for macro_378 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_378 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_378"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_add_casp as atomic_add , select_le_or_be ! ("adds x4, x6, {val_lo}" , "adds x5, x7, {val_hi}") , select_le_or_be ! ("adc x5, x7, {val_hi}" , "adc x4, x6, {val_lo}") , }
};
}
