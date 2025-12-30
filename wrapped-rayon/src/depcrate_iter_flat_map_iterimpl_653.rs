// Generated macro for impl_653 (impl)
macro_rules! Depcrate_iter_flat_map_iterimpl_653 {
() => {
// Module: crate::iter::flat_map_iter
// Provides: {"impl_653"}
// Dependencies: {}
impl < I , F > FlatMapIter < I , F > { # [doc = " Creates a new `FlatMapIter` iterator."] pub (super) fn new (base : I , map_op : F) -> Self { FlatMapIter { base , map_op } } }
};
}
