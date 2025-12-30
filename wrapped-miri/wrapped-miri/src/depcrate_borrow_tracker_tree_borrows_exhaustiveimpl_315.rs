// Generated macro for impl_315 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_315 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_315"}
// Dependencies: {}
impl < T > Exhaustive for [T ; 3] where T : Exhaustive + Clone + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (< [T ; 2] > :: exhaustive () . flat_map (| [t1 , t2] | T :: exhaustive () . map (move | t3 | [t1 . clone () , t2 . clone () , t3])) ,) } }
};
}
