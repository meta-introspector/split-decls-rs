macro_rules! deps {
    () => {
        Vectorization!();
    };
}

macro_rules! vectorization_support_no_cache_arm {
    () => {
        deps!();
        # [cfg (target_arch = "aarch64")] # [cold] fn vectorization_support_no_cache_arm () -> Vectorization { # [cfg (feature = "std")] if std :: arch :: is_aarch64_feature_detected ! ("neon") { return Vectorization :: Neon ; } # [cfg (target_feature = "neon")] return Vectorization :: Neon ; # [allow (unreachable_code)] Vectorization :: None }
    };
}

vectorization_support_no_cache_arm!()