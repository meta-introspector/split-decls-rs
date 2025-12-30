// Generated macro for impl_169 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrowsimpl_169 {
() => {
// Module: crate::borrow_tracker::stacked_borrows
// Provides: {"impl_169"}
// Dependencies: {}
impl VisitProvenance for Stacks { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { for tag in self . exposed_tags . iter () . copied () { visit (None , Some (tag)) ; } } }
};
}
