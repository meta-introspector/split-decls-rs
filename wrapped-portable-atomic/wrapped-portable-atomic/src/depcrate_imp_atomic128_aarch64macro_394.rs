// Generated macro for macro_394 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_394 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_394"}
// Dependencies: {}
atomic_rmw_cas_2 ! { _atomic_neg_casp as atomic_neg , select_le_or_be ! ("negs x4, x6" , "negs x5, x7") , select_le_or_be ! ("ngc x5, x7" , "ngc x4, x6") , }
};
}
