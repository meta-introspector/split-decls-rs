// Generated macro for has_kuser_cmpxchg64 (function)
macro_rules! Depcrate_imp_atomic64_arm_linuxhas_kuser_cmpxchg64 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"has_kuser_cmpxchg64"}
// Dependencies: {}
# [inline] fn has_kuser_cmpxchg64 () -> bool { if cfg ! (portable_atomic_test_detect_false) { return false ; } __kuser_helper_version () >= 5 }
};
}
