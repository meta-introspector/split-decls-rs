macro_rules! UnknownCrateTypesSub {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_suggestion , code = r#""{candidate}""# , applicability = "maybe-incorrect")] pub (crate) struct UnknownCrateTypesSub { # [primary_span] pub span : Span , pub candidate : Symbol , }
    };
}

UnknownCrateTypesSub!();