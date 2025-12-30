// Generated macro for impl_317 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_317 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_317"}
// Dependencies: {}
impl < T > Exhaustive for [T ; 5] where T : Exhaustive + Clone + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (< [T ; 2] > :: exhaustive () . flat_map (| [t1 , t2] | { < [T ; 3] > :: exhaustive () . map (move | [t3 , t4 , t5] | [t1 . clone () , t2 . clone () , t3 , t4 , t5]) })) } }
};
}
