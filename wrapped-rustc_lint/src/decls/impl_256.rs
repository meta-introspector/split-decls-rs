macro_rules! deps {
    () => {
        ImplTraitOvercapturesLint!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < 'a > LintDiagnostic < 'a , () > for ImplTraitOvercapturesLint < '_ > { fn decorate_lint < 'b > (self , diag : & 'b mut rustc_errors :: Diag < 'a , () >) { diag . primary_message (fluent :: lint_impl_trait_overcaptures) ; diag . arg ("self_ty" , self . self_ty . to_string ()) . arg ("num_captured" , self . num_captured) . span_note (self . uncaptured_spans , fluent :: lint_note) . note (fluent :: lint_note2) ; if let Some (suggestion) = self . suggestion { suggestion . add_to_diag (diag) ; } } }
    };
}

impl_256!();