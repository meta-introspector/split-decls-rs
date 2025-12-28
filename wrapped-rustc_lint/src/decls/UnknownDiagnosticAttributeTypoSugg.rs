macro_rules! UnknownDiagnosticAttributeTypoSugg {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_unknown_diagnostic_attribute_typo_sugg , style = "verbose" , code = "{typo_name}" , applicability = "machine-applicable")] pub (crate) struct UnknownDiagnosticAttributeTypoSugg { # [primary_span] pub span : Span , pub typo_name : Symbol , }
    };
}

UnknownDiagnosticAttributeTypoSugg!();