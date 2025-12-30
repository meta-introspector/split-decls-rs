// Generated macro for impl_678 (impl)
macro_rules! Depcrate_iter_flatten_iterimpl_678 {
() => {
// Module: crate::iter::flatten_iter
// Provides: {"impl_678"}
// Dependencies: {}
impl < I > ParallelIterator for FlattenIter < I > where I : ParallelIterator < Item : IntoIterator < Item : Send > > , { type Item = < I :: Item as IntoIterator > :: Item ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let consumer = FlattenIterConsumer :: new (consumer) ; self . base . drive_unindexed (consumer) } }
};
}
