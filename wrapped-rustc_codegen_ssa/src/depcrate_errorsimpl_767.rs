// Generated macro for impl_767 (impl)
macro_rules! Depcrate_errorsimpl_767 {
() => {
// Module: crate::errors
// Provides: {"impl_767"}
// Dependencies: {}
impl < G : EmissionGuarantee > Diagnostic < '_ , G > for TargetFeatureDisableOrEnable < '_ > { fn into_diag (self , dcx : DiagCtxtHandle < '_ > , level : Level) -> Diag < '_ , G > { let mut diag = Diag :: new (dcx , level , fluent :: codegen_ssa_target_feature_disable_or_enable) ; if let Some (span) = self . span { diag . span (span) ; } ; if let Some (missing_features) = self . missing_features { diag . subdiagnostic (missing_features) ; } diag . arg ("features" , self . features . join (", ")) ; diag } }
};
}
