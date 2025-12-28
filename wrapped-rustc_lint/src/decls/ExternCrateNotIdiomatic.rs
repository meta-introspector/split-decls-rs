macro_rules! ExternCrateNotIdiomatic {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_extern_crate_not_idiomatic)] pub (crate) struct ExternCrateNotIdiomatic { # [suggestion (style = "verbose" , code = "{code}" , applicability = "machine-applicable")] pub span : Span , pub code : & 'static str , }
    };
}

ExternCrateNotIdiomatic!()