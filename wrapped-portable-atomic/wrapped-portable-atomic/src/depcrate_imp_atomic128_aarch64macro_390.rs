// Generated macro for macro_390 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_390 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_390"}
// Dependencies: {}
atomic_rmw_cas_3 ! { _atomic_xor_casp as atomic_xor , "eor x4, x6, {val_lo}" , "eor x5, x7, {val_hi}" , }
};
}
