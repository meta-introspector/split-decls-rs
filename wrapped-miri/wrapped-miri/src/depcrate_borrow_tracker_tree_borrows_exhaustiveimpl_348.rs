// Generated macro for impl_348 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_348 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_348"}
// Dependencies: {}
impl < T > Exhaustive for [T ; 1] where T : Exhaustive + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (T :: exhaustive () . map (| t | [t])) } }
};
}
