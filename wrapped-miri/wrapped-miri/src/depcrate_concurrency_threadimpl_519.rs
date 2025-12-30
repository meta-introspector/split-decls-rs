// Generated macro for impl_519 (impl)
macro_rules! Depcrate_concurrency_threadimpl_519 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_519"}
// Dependencies: {}
impl VisitProvenance for Frame < '_ , Provenance , FrameExtra < '_ > > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let Frame { return_place , locals , extra , .. } = self ; return_place . visit_provenance (visit) ; for local in locals . iter () { match local . as_mplace_or_imm () { None => { } Some (Either :: Left ((ptr , meta))) => { ptr . visit_provenance (visit) ; meta . visit_provenance (visit) ; } Some (Either :: Right (imm)) => { imm . visit_provenance (visit) ; } } } extra . visit_provenance (visit) ; } }
};
}
