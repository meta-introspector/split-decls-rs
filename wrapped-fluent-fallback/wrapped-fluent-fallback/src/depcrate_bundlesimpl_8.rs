// Generated macro for impl_8 (impl)
macro_rules! Depcrate_bundlesimpl_8 {
() => {
// Module: crate::bundles
// Provides: {"impl_8"}
// Dependencies: {}
impl < G > Bundles < G > where G : BundleGenerator , G :: Iter : BundleIterator , { pub fn prefetch_sync (& self) { match & self . 0 { BundlesInner :: Iter (iter) => iter . prefetch () , BundlesInner :: Stream (_) => panic ! ("Can't prefetch a sync bundle set asynchronously") , } } }
};
}
