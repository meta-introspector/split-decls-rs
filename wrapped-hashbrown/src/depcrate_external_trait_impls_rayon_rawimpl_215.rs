// Generated macro for impl_215 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_215 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_215"}
// Dependencies: {}
impl < T : Send , A : Allocator + Send > ParallelIterator for RawIntoParIter < T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let iter = unsafe { self . table . iter () . iter } ; let _guard = guard (self . table . into_allocation () , | alloc | { if let Some ((ptr , layout , ref alloc)) = * alloc { unsafe { alloc . deallocate (ptr , layout) ; } } }) ; let producer = ParDrainProducer { iter } ; plumbing :: bridge_unindexed (producer , consumer) } }
};
}
