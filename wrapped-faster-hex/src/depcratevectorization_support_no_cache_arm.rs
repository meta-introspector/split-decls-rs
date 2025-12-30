// Generated macro for vectorization_support_no_cache_arm (function)
macro_rules! Depcratevectorization_support_no_cache_arm {
() => {
// Module: crate
// Provides: {"vectorization_support_no_cache_arm"}
// Dependencies: {}
# [cfg (target_arch = "aarch64")] # [cold] fn vectorization_support_no_cache_arm () -> Vectorization { # [cfg (feature = "std")] if std :: arch :: is_aarch64_feature_detected ! ("neon") { return Vectorization :: Neon ; } # [cfg (target_feature = "neon")] return Vectorization :: Neon ; # [allow (unreachable_code)] Vectorization :: None }
};
}
