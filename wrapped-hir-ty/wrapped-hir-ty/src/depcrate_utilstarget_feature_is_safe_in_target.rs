// Generated macro for target_feature_is_safe_in_target (function)
macro_rules! Depcrate_utilstarget_feature_is_safe_in_target {
() => {
// Module: crate::utils
// Provides: {"target_feature_is_safe_in_target"}
// Dependencies: {}
pub fn target_feature_is_safe_in_target (target : & TargetData) -> TargetFeatureIsSafeInTarget { match target . arch { target :: Arch :: Wasm32 | target :: Arch :: Wasm64 => TargetFeatureIsSafeInTarget :: Yes , _ => TargetFeatureIsSafeInTarget :: No , } }
};
}
