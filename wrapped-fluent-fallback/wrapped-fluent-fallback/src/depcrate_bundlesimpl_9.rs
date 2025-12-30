// Generated macro for impl_9 (impl)
macro_rules! Depcrate_bundlesimpl_9 {
() => {
// Module: crate::bundles
// Provides: {"impl_9"}
// Dependencies: {}
impl < G > Bundles < G > where G : BundleGenerator , G :: Stream : BundleStream , { pub async fn prefetch_async (& self) { match & self . 0 { BundlesInner :: Iter (_) => panic ! ("Can't prefetch a async bundle set synchronously") , BundlesInner :: Stream (stream) => stream . prefetch () . await , } } }
};
}
