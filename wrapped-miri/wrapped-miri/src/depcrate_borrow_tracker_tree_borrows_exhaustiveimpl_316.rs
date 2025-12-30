// Generated macro for impl_316 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_316 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_316"}
// Dependencies: {}
impl < T > Exhaustive for [T ; 4] where T : Exhaustive + Clone + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (< [T ; 2] > :: exhaustive () . flat_map (| [t1 , t2] | { < [T ; 2] > :: exhaustive () . map (move | [t3 , t4] | [t1 . clone () , t2 . clone () , t3 , t4]) })) } }
};
}
