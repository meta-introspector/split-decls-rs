macro_rules! NoopMethodCallDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_noop_method_call)] # [note] pub (crate) struct NoopMethodCallDiag < 'a > { pub method : Ident , pub orig_ty : Ty < 'a > , pub trait_ : Symbol , # [suggestion (code = "" , applicability = "machine-applicable")] pub label : Span , # [suggestion (lint_derive_suggestion , code = "#[derive(Clone)]\n" , applicability = "maybe-incorrect")] pub suggest_derive : Option < Span > , }
    };
}

NoopMethodCallDiag!();