// Generated macro for impl_568 (impl)
macro_rules! Depcrate_session_diagnosticsimpl_568 {
() => {
// Module: crate::session_diagnostics
// Provides: {"impl_568"}
// Dependencies: {}
impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for UnsupportedLiteral { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , match self . reason { UnsupportedLiteralReason :: Generic => { fluent :: attr_parsing_unsupported_literal_generic } UnsupportedLiteralReason :: CfgString => { fluent :: attr_parsing_unsupported_literal_cfg_string } UnsupportedLiteralReason :: CfgBoolean => { fluent :: attr_parsing_unsupported_literal_cfg_boolean } } ,) ; diag . span (self . span) ; diag . code (E0565) ; if self . is_bytestr { diag . span_suggestion (self . start_point_span , fluent :: attr_parsing_unsupported_literal_suggestion , "" , Applicability :: MaybeIncorrect ,) ; } diag } }
};
}
