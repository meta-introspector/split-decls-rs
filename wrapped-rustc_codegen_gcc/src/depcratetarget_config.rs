// Generated macro for target_config (function)
macro_rules! Depcratetarget_config {
() => {
// Module: crate
// Provides: {"target_config"}
// Dependencies: {}
# [doc = " Returns the features that should be set in `cfg(target_feature)`."] fn target_config (sess : & Session , target_info : & LockedTargetInfo) -> TargetConfig { let (unstable_target_features , target_features) = cfg_target_feature (sess , | feature | { if feature == "neon" { return false ; } target_info . cpu_supports (feature) }) ; let has_reliable_f16 = target_info . supports_target_dependent_type (CType :: Float16) ; let has_reliable_f128 = target_info . supports_target_dependent_type (CType :: Float128) ; TargetConfig { target_features , unstable_target_features , has_reliable_f16 , has_reliable_f16_math : has_reliable_f16 , has_reliable_f128 , has_reliable_f128_math : has_reliable_f128 , } }
};
}
