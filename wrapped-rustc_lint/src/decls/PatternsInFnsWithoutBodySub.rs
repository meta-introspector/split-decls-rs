macro_rules! PatternsInFnsWithoutBodySub {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_remove_mut_from_pattern , code = "{ident}" , applicability = "machine-applicable")] pub (crate) struct PatternsInFnsWithoutBodySub { # [primary_span] pub span : Span , pub ident : Ident , }
    };
}

PatternsInFnsWithoutBodySub!();