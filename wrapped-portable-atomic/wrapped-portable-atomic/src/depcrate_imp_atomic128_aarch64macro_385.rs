// Generated macro for macro_385 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_385 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_385"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_nand_casp as atomic_nand , "and x4, x6, {val_lo}" , "and x5, x7, {val_hi}" , "mvn x4, x4" , "mvn x5, x5" , }
};
}
