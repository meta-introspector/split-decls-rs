// Generated macro for impl_766 (impl)
macro_rules! Depcrate_provider_skeleton_helpersimpl_766 {
() => {
// Module: crate::provider::skeleton::helpers
// Provides: {"impl_766"}
// Dependencies: {}
impl SkeletonQuality { # [doc = " Returns the worst possible quality measure."] pub fn worst () -> SkeletonQuality { SkeletonQuality (u32 :: MAX) } # [doc = " Returns the best possible quality measure."] pub fn best () -> SkeletonQuality { SkeletonQuality (0) } # [doc = " Returns whether this is an \"excellent\" match by an arbitrary definition."] pub fn is_excellent_match (self) -> bool { self . 0 < GLUE_DISTANCE } }
};
}
