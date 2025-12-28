macro_rules! RenamedLintSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum RenamedLintSuggestion < 'a > { # [suggestion (lint_suggestion , code = "{replace}" , applicability = "machine-applicable")] WithSpan { # [primary_span] suggestion : Span , replace : & 'a str , } , # [help (lint_help)] WithoutSpan { replace : & 'a str } , }
    };
}

RenamedLintSuggestion!()