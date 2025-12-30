// Generated macro for impl_609 (impl)
macro_rules! Depcrate_errorsimpl_609 {
() => {
// Module: crate::errors
// Provides: {"impl_609"}
// Dependencies: {}
impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for MissingTypeParams { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut err = Diag :: new (dcx , level , fluent :: hir_analysis_missing_type_params) ; err . span (self . span) ; err . code (E0393) ; err . arg ("parameterCount" , self . missing_type_params . len ()) ; err . arg ("parameters" , self . missing_type_params . iter () . map (| n | format ! ("`{n}`")) . collect :: < Vec < _ > > () . join (", ") ,) ; err . span_label (self . def_span , fluent :: hir_analysis_label) ; let mut suggested = false ; if let Some (snippet) = self . span_snippet && self . empty_generic_args { if snippet . ends_with ('>') { } else { err . span_suggestion_verbose (self . span . shrink_to_hi () , fluent :: hir_analysis_suggestion , format ! ("<{}>" , self . missing_type_params . iter () . map (| n | n . to_string ()) . collect ::< Vec < _ >> () . join (", ")) , Applicability :: HasPlaceholders ,) ; suggested = true ; } } if ! suggested { err . span_label (self . span , fluent :: hir_analysis_no_suggestion_label) ; } err . note (fluent :: hir_analysis_note) ; err } }
};
}
