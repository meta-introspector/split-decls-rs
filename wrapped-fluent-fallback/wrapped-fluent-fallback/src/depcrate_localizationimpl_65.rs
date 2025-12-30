// Generated macro for impl_65 (impl)
macro_rules! Depcrate_localizationimpl_65 {
() => {
// Module: crate::localization
// Provides: {"impl_65"}
// Dependencies: {}
impl < G , P > Localization < G , P > where G : BundleGenerator < LocalesIter = P :: Iter > , G :: Iter : BundleIterator , P : LocalesProvider , { pub fn prefetch_sync (& mut self) { let bundles = self . bundles () ; bundles . prefetch_sync () ; } }
};
}
