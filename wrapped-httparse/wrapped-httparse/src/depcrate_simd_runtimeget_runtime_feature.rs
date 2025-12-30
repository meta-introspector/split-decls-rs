// Generated macro for get_runtime_feature (function)
macro_rules! Depcrate_simd_runtimeget_runtime_feature {
() => {
// Module: crate::simd::runtime
// Provides: {"get_runtime_feature"}
// Dependencies: {}
# [inline] fn get_runtime_feature () -> u8 { let mut feature = RUNTIME_FEATURE . load (Ordering :: Relaxed) ; if feature == 0 { feature = detect_runtime_feature () ; RUNTIME_FEATURE . store (feature , Ordering :: Relaxed) ; } feature }
};
}
