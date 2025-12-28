macro_rules! deps {
    () => {
        MismatchedLifetimeSyntaxes!();
        LifetimeSyntaxCategories!();
    };
}

macro_rules! impl_644 {
    () => {
        deps!();
        impl < 'a , G : EmissionGuarantee > LintDiagnostic < 'a , G > for MismatchedLifetimeSyntaxes { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , G >) { let counts = self . inputs . len () + self . outputs . len () ; let message = match counts { LifetimeSyntaxCategories { hidden : 0 , elided : 0 , named : 0 } => { panic ! ("No lifetime mismatch detected") } LifetimeSyntaxCategories { hidden : _ , elided : _ , named : 0 } => { fluent :: lint_mismatched_lifetime_syntaxes_hiding_while_elided } LifetimeSyntaxCategories { hidden : _ , elided : 0 , named : _ } => { fluent :: lint_mismatched_lifetime_syntaxes_hiding_while_named } LifetimeSyntaxCategories { hidden : 0 , elided : _ , named : _ } => { fluent :: lint_mismatched_lifetime_syntaxes_eliding_while_named } LifetimeSyntaxCategories { hidden : _ , elided : _ , named : _ } => { fluent :: lint_mismatched_lifetime_syntaxes_hiding_and_eliding_while_named } } ; diag . primary_message (message) ; for s in self . inputs . hidden { diag . span_label (s , fluent :: lint_mismatched_lifetime_syntaxes_input_hidden) ; } for s in self . inputs . elided { diag . span_label (s , fluent :: lint_mismatched_lifetime_syntaxes_input_elided) ; } for s in self . inputs . named { diag . span_label (s , fluent :: lint_mismatched_lifetime_syntaxes_input_named) ; } for s in self . outputs . hidden { diag . span_label (s , fluent :: lint_mismatched_lifetime_syntaxes_output_hidden) ; } for s in self . outputs . elided { diag . span_label (s , fluent :: lint_mismatched_lifetime_syntaxes_output_elided) ; } for s in self . outputs . named { diag . span_label (s , fluent :: lint_mismatched_lifetime_syntaxes_output_named) ; } diag . help (fluent :: lint_mismatched_lifetime_syntaxes_help) ; let mut suggestions = self . suggestions . into_iter () ; if let Some (s) = suggestions . next () { diag . subdiagnostic (s) ; for mut s in suggestions { s . make_optional_alternative () ; diag . subdiagnostic (s) ; } } } }
    };
}

impl_644!();