// Generated macro for debug_assert_cmpxchg16b (macro)
macro_rules! Depcrate_imp_atomic128_x86_64debug_assert_cmpxchg16b {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"debug_assert_cmpxchg16b"}
// Dependencies: {}
macro_rules ! debug_assert_cmpxchg16b { () => { # [cfg (not (any (target_feature = "cmpxchg16b" , portable_atomic_target_feature = "cmpxchg16b" ,)))] { debug_assert ! (detect :: detect () . cmpxchg16b ()) ; } } ; }
};
}
