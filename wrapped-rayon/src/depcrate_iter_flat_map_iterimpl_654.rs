// Generated macro for impl_654 (impl)
macro_rules! Depcrate_iter_flat_map_iterimpl_654 {
() => {
// Module: crate::iter::flat_map_iter
// Provides: {"impl_654"}
// Dependencies: {}
impl < I , F , SI > ParallelIterator for FlatMapIter < I , F > where I : ParallelIterator , F : Fn (I :: Item) -> SI + Sync + Send , SI : IntoIterator < Item : Send > , { type Item = SI :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FlatMapIterConsumer :: new (consumer , & self . map_op) ; self . base . drive_unindexed (consumer) } }
};
}
