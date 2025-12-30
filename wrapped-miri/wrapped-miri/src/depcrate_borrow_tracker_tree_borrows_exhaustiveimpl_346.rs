// Generated macro for impl_346 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_exhaustiveimpl_346 {
() => {
// Module: crate::borrow_tracker::tree_borrows::exhaustive
// Provides: {"impl_346"}
// Dependencies: {}
impl < T > Exhaustive for Option < T > where T : Exhaustive + 'static , { fn exhaustive () -> Box < dyn Iterator < Item = Self > > { Box :: new (std :: iter :: once (None) . chain (T :: exhaustive () . map (Some))) } }
};
}
