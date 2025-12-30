// Generated macro for tests (module)
macro_rules! Depcrate_imp_atomic64_arm_linuxtests {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"tests"}
// Dependencies: {}
# [allow (clippy :: alloc_instead_of_core , clippy :: std_instead_of_alloc , clippy :: std_instead_of_core , clippy :: undocumented_unsafe_blocks , clippy :: wildcard_imports)] # [cfg (test)] mod tests { use super :: * ; # [test] fn kuser_helper_version () { let version = __kuser_helper_version () ; assert ! (version >= 5 , "{:?}" , version) ; assert_eq ! (version , unsafe { crate :: utils :: ptr :: with_exposed_provenance ::< i32 > (KUSER_HELPER_VERSION) . read () }) ; } test_atomic_int ! (i64) ; test_atomic_int ! (u64) ; stress_test ! (u64) ; }
};
}
