// Generated macro for impl_337 (impl)
macro_rules! Depcrate_errorsimpl_337 {
() => {
// Module: crate::errors
// Provides: {"impl_337"}
// Dependencies: {}
impl Subdiagnostic for OverruledAttributeSub { fn add_to_diag < G : EmissionGuarantee > (self , diag : & mut Diag < '_ , G >) { match self { OverruledAttributeSub :: DefaultSource { id } => { diag . note (fluent :: lint_default_source) ; diag . arg ("id" , id) ; } OverruledAttributeSub :: NodeSource { span , reason } => { diag . span_label (span , fluent :: lint_node_source) ; if let Some (rationale) = reason { # [allow (rustc :: untranslatable_diagnostic)] diag . note (rationale . to_string ()) ; } } OverruledAttributeSub :: CommandLineSource => { diag . note (fluent :: lint_command_line_source) ; } } } }
};
}
