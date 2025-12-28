macro_rules! UnusedLifetime {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_lifetime)] pub (crate) struct UnusedLifetime { # [suggestion (code = "" , applicability = "machine-applicable")] pub deletion_span : Option < Span > , pub ident : Ident , }
    };
}

UnusedLifetime!();