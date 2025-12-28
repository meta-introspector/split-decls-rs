macro_rules! NoDefaultVariantSugg {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (builtin_macros_suggestion , code = "#[default] " , applicability = "maybe-incorrect")] pub (crate) struct NoDefaultVariantSugg { # [primary_span] pub (crate) span : Span , }
    };
}

NoDefaultVariantSugg!();