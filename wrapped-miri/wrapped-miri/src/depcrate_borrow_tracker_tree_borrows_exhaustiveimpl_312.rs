// Generated macro for impl_312 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_312 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_312"}
// Dependencies: {}
impl < T1 , T2 > Exhaustive for (T1 , T2) where T1 : Exhaustive + Clone + 'static , T2 : Exhaustive + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (T1 :: exhaustive () . flat_map (| t1 | T2 :: exhaustive () . map (move | t2 | (t1 . clone () , t2)))) } }
};
}
