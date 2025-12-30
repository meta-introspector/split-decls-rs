// Generated macro for IMP_ARM_LINUX (const)
macro_rules! Depcrate_tests_helperIMP_ARM_LINUX {
() => {
// Module: crate::tests::helper
// Provides: {"IMP_ARM_LINUX"}
// Dependencies: {}
pub (crate) const IMP_ARM_LINUX : bool = cfg ! (all (target_arch = "arm" , any (target_os = "linux" , target_os = "android") , any (not (any (target_feature = "v6" , atomic_maybe_uninit_target_feature = "v6")) , atomic_maybe_uninit_test_prefer_kuser_cmpxchg ,) , not (any (target_feature = "v8" , atomic_maybe_uninit_target_feature = "v8" , target_feature = "v8m" , atomic_maybe_uninit_target_feature = "v8m" ,)) ,)) ;
};
}
