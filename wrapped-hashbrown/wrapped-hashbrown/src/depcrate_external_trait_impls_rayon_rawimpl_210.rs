// Generated macro for impl_210 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_210 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_210"}
// Dependencies: {}
impl < T > ParallelIterator for RawParIter < T > { type Item = Bucket < T > ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let producer = ParIterProducer { iter : self . iter } ; plumbing :: bridge_unindexed (producer , consumer) } }
};
}
