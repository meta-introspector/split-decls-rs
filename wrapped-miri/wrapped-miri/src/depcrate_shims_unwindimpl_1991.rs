// Generated macro for impl_1991 (impl)
macro_rules! Depcrate_shims_unwindimpl_1991 {
() => {
// Module: crate::shims::unwind
// Provides: {"impl_1991"}
// Dependencies: {}
impl VisitProvenance for CatchUnwindData < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let CatchUnwindData { catch_fn , data , dest , ret : _ } = self ; catch_fn . visit_provenance (visit) ; data . visit_provenance (visit) ; dest . visit_provenance (visit) ; } }
};
}
