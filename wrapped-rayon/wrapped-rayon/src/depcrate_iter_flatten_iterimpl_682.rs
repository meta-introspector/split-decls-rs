// Generated macro for impl_682 (impl)
macro_rules! Depcrate_iter_flatten_iterimpl_682 {
() => {
// Module: crate::iter::flatten_iter
// Provides: {"impl_682"}
// Dependencies: {}
impl < T , C > UnindexedConsumer < T > for FlattenIterConsumer < C > where C : UnindexedConsumer < T :: Item > , T : IntoIterator , { fn split_off_left (& self) -> Self { FlattenIterConsumer :: new (self . base . split_off_left ()) } fn to_reducer (& self) -> Self :: Reducer { self . base . to_reducer () } }
};
}
