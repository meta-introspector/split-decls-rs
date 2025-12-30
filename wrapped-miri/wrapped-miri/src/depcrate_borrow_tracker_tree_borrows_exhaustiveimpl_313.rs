// Generated macro for impl_313 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_313 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_313"}
// Dependencies: {}
impl < T > Exhaustive for [T ; 1] where T : Exhaustive + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (T :: exhaustive () . map (| t | [t])) } }
};
}
