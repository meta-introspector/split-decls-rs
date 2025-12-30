// Generated macro for impl_385 (impl)
macro_rules! Depcrate_borrow_trackerimpl_385 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_385"}
// Dependencies: {}
impl VisitProvenance for AllocState { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let _trace = enter_trace_span ! (borrow_tracker :: visit_provenance) ; match self { AllocState :: StackedBorrows (sb) => sb . visit_provenance (visit) , AllocState :: TreeBorrows (tb) => tb . visit_provenance (visit) , } } }
};
}
