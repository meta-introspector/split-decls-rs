macro_rules! deps {
    () => {
        NonFmtPanicUnused!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl < 'a > LintDiagnostic < 'a , () > for NonFmtPanicUnused { fn decorate_lint < 'b > (self , diag : & 'b mut Diag < 'a , () >) { diag . primary_message (fluent :: lint_non_fmt_panic_unused) ; diag . arg ("count" , self . count) ; diag . note (fluent :: lint_note) ; if let Some (span) = self . suggestion { diag . span_suggestion (span . shrink_to_hi () , fluent :: lint_add_args_suggestion , ", ..." , Applicability :: HasPlaceholders ,) ; diag . span_suggestion (span . shrink_to_lo () , fluent :: lint_add_fmt_suggestion , "\"{}\", " , Applicability :: MachineApplicable ,) ; } } }
    };
}

impl_479!()