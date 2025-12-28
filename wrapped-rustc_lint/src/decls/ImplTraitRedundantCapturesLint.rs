macro_rules! ImplTraitRedundantCapturesLint {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_impl_trait_redundant_captures)] struct ImplTraitRedundantCapturesLint { # [suggestion (lint_suggestion , code = "" , applicability = "machine-applicable")] capturing_span : Span , }
    };
}

ImplTraitRedundantCapturesLint!();