// Generated macro for impl_333 (impl)
macro_rules! Depcrate_borrow_trackerimpl_333 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_333"}
// Dependencies: {}
impl VisitProvenance for FrameState { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { for (id , tag) in & self . protected_tags { visit (Some (* id) , Some (* tag)) ; } } }
};
}
