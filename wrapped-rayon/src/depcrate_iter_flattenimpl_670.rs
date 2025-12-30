// Generated macro for impl_670 (impl)
macro_rules! Depcrate_iter_flattenimpl_670 {
() => {
// Module: crate::iter::flatten
// Provides: {"impl_670"}
// Dependencies: {}
impl < T , C > UnindexedConsumer < T > for FlattenConsumer < C > where C : UnindexedConsumer < T :: Item > , T : IntoParallelIterator , { fn split_off_left (& self) -> Self { FlattenConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
