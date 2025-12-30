// Generated macro for impl_470 (impl)
macro_rules! Depcrate_impl_trait_overcapturesimpl_470 {
() => {
// Module: crate::impl_trait_overcaptures
// Provides: {"impl_470"}
// Dependencies: {}
impl < 'a > LintDiagnostic < 'a , () > for ImplTraitOvercapturesLint < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut rustc_errors :: Diag < 'a , () >) { diag . primary_message (fluent :: lint_impl_trait_overcaptures) ; diag . arg ("self_ty" , self . self_ty . to_string ()) . arg ("num_captured" , self . num_captured) . span_note (self . uncaptured_spans , fluent :: lint_note) . note (fluent :: lint_note2) ; if let Some (suggestion) = self . suggestion { suggestion . add_to_diag (diag) ; } } }
};
}
