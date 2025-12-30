// Generated macro for impl_1781 (impl)
macro_rules! Depcrate_shims_unwindimpl_1781 {
() => {
// Module: crate::shims::unwind
// Provides: {"impl_1781"}
// Dependencies: {}
impl VisitProvenance for CatchUnwindData < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let CatchUnwindData { catch_fn , data , dest , ret : _ } = self ; catch_fn . visit_provenance (visit) ; data . visit_provenance (visit) ; dest . visit_provenance (visit) ; } }
};
}
