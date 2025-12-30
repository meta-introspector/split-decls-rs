// Generated macro for impl_658 (impl)
macro_rules! Depcrate_iter_flat_map_iterimpl_658 {
() => {
// Module: crate::iter::flat_map_iter
// Provides: {"impl_658"}
// Dependencies: {}
impl < 'f , T , U , C , F > UnindexedConsumer < T > for FlatMapIterConsumer < 'f , C , F > where C : UnindexedConsumer < U :: Item > , F : Fn (T) -> U + Sync , U : IntoIterator , { fn split_off_left (& self) -> Self { FlatMapIterConsumer :: new (self . base . split_off_left () , self . map_op) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
