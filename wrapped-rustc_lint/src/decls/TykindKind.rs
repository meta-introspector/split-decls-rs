macro_rules! TykindKind {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_tykind_kind)] pub (crate) struct TykindKind { # [suggestion (code = "ty" , applicability = "maybe-incorrect")] pub suggestion : Span , }
    };
}

TykindKind!();