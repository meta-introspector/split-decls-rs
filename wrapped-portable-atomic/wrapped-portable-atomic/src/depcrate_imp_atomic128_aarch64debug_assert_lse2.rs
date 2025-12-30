// Generated macro for debug_assert_lse2 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64debug_assert_lse2 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"debug_assert_lse2"}
// Dependencies: {}
# [cfg (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2" , not (portable_atomic_no_outline_atomics) ,))] # [rustfmt :: skip] macro_rules ! debug_assert_lse2 { () => { # [cfg (all (not (portable_atomic_no_outline_atomics) , any (all (target_os = "linux" , any (target_env = "gnu" , all (target_env = "musl" , any (not (target_feature = "crt-static") , feature = "std") ,) , target_env = "ohos" , all (target_env = "uclibc" , not (target_feature = "crt-static")) , portable_atomic_outline_atomics ,) ,) , target_os = "android" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , all (target_os = "illumos" , portable_atomic_outline_atomics) ,) ,))] # [cfg (not (any (target_feature = "lse2" , portable_atomic_target_feature = "lse2")))] { debug_assert ! (detect :: detect () . lse2 ()) ; } } ; }
};
}
