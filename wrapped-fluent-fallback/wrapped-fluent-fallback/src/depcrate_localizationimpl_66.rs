// Generated macro for impl_66 (impl)
macro_rules! Depcrate_localizationimpl_66 {
() => {
// Module: crate::localization
// Provides: {"impl_66"}
// Dependencies: {}
impl < G , P > Localization < G , P > where G : BundleGenerator < LocalesIter = P :: Iter > , G :: Stream : BundleStream , P : LocalesProvider , { pub async fn prefetch_async (& mut self) { let bundles = self . bundles () ; bundles . prefetch_async () . await ; } }
};
}
