// Generated macro for impl_1059 (impl)
macro_rules! Depcrate_provenance_gcimpl_1059 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1059"}
// Dependencies: {}
impl VisitProvenance for OpTy < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self . as_mplace_or_imm () { Either :: Left (mplace) => mplace . visit_provenance (visit) , Either :: Right (imm) => imm . visit_provenance (visit) , } } }
};
}
