// Generated macro for debug_assert_zacas (macro)
macro_rules! Depcrate_imp_atomic64_riscv32debug_assert_zacas {
() => {
// Module: crate::imp::atomic64::riscv32
// Provides: {"debug_assert_zacas"}
// Dependencies: {}
macro_rules ! debug_assert_zacas { () => { # [cfg (not (any (target_feature = "zacas" , portable_atomic_target_feature = "zacas")))] { debug_assert ! (detect :: detect () . zacas ()) ; } } ; }
};
}
