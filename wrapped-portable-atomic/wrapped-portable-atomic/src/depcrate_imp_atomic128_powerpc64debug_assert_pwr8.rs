// Generated macro for debug_assert_pwr8 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64debug_assert_pwr8 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"debug_assert_pwr8"}
// Dependencies: {}
macro_rules ! debug_assert_pwr8 { () => { # [cfg (not (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,)))] { debug_assert ! (detect :: detect () . quadword_atomics ()) ; } } ; }
};
}
