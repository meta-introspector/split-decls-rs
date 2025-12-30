// Generated macro for impl_219 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_rawimpl_219 {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"impl_219"}
// Dependencies: {}
impl < T : Send , A : Allocator > ParallelIterator for RawParDrain < '_ , T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let _guard = guard (self . table , | table | unsafe { table . as_mut () . clear_no_drop () ; }) ; let iter = unsafe { self . table . as_ref () . iter () . iter } ; mem :: forget (self) ; let producer = ParDrainProducer { iter } ; plumbing :: bridge_unindexed (producer , consumer) } }
};
}
