macro_rules! TyQualified {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_ty_qualified)] pub (crate) struct TyQualified { pub ty : String , # [suggestion (code = "{ty}" , applicability = "maybe-incorrect")] pub suggestion : Span , }
    };
}

TyQualified!();