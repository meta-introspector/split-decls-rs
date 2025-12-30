// Generated macro for BundlesInner (enum)
macro_rules! Depcrate_bundlesBundlesInner {
() => {
// Module: crate::bundles
// Provides: {"BundlesInner"}
// Dependencies: {}
pub enum BundlesInner < G > where G : BundleGenerator , { Iter (Cache < G :: Iter , G :: Resource >) , Stream (AsyncCache < G :: Stream , G :: Resource >) , }
};
}
