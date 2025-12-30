// Generated macro for macro_391 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64macro_391 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"macro_391"}
// Dependencies: {}
atomic_rmw_ll_sc_2 ! { _atomic_not_ldxp_stxp as atomic_not (preserves_flags) , "mvn {new_lo}, {prev_lo}" , "mvn {new_hi}, {prev_hi}" , }
};
}
