macro_rules! deps {
    () => {
        TargetFeatureIsSafeInTarget!();
    };
}

macro_rules! target_feature_is_safe_in_target {
    () => {
        deps!();
        pub fn target_feature_is_safe_in_target (target : & TargetData) -> TargetFeatureIsSafeInTarget { match target . arch { target :: Arch :: Wasm32 | target :: Arch :: Wasm64 => TargetFeatureIsSafeInTarget :: Yes , _ => TargetFeatureIsSafeInTarget :: No , } }
    };
}

target_feature_is_safe_in_target!();