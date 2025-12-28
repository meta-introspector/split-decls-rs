macro_rules! AbsPathWithModuleSugg {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_suggestion , code = "{replacement}")] pub (crate) struct AbsPathWithModuleSugg { # [primary_span] pub span : Span , # [applicability] pub applicability : Applicability , pub replacement : String , }
    };
}

AbsPathWithModuleSugg!()