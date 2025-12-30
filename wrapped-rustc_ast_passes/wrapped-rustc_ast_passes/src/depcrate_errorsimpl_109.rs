// Generated macro for impl_109 (impl)
macro_rules! Depcrate_errorsimpl_109 {
() => {
// Module: crate::errors
// Provides: {"impl_109"}
// Dependencies: {}
impl Subdiagnostic for StableFeature { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { diag . arg ("name" , self . name) ; diag . arg ("since" , self . since) ; diag . help (fluent :: ast_passes_stable_since) ; } }
};
}
