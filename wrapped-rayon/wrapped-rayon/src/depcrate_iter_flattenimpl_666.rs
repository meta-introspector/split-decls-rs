// Generated macro for impl_666 (impl)
macro_rules! Depcrate_iter_flattenimpl_666 {
() => {
// Module: crate::iter::flatten
// Provides: {"impl_666"}
// Dependencies: {}
impl < I > ParallelIterator for Flatten < I > where I : ParallelIterator < Item : IntoParallelIterator > , { type Item = < I :: Item as IntoParallelIterator > :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FlattenConsumer :: new (consumer) ; self . base . drive_unindexed (consumer) } }
};
}
