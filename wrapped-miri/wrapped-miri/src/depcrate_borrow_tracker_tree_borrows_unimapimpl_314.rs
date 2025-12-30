// Generated macro for impl_314 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_unimapimpl_314 {
() => {
// Module: crate::borrow_tracker::tree_borrows::unimap
// Provides: {"impl_314"}
// Dependencies: {}
impl < V : PartialEq > PartialEq for UniValMap < V > { # [doc = " 2023-05: We found that using `equivalent` rather than `identical`"] # [doc = " in the equality testing of the `RangeMap` is neutral for most"] # [doc = " benchmarks, while being quite beneficial for `zip-equal`"] # [doc = " and to a lesser extent for `unicode`, `slice-get-unchecked` and"] # [doc = " `backtraces` as well."] fn eq (& self , other : & Self) -> bool { self . equivalent (other) } }
};
}
