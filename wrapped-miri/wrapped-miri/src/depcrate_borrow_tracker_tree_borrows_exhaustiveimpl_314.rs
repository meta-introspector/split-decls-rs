// Generated macro for impl_314 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_314 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_314"}
// Dependencies: {}
impl < T > Exhaustive for [T ; 2] where T : Exhaustive + Clone + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (T :: exhaustive () . flat_map (| t1 | T :: exhaustive () . map (move | t2 | [t1 . clone () , t2]))) } }
};
}
