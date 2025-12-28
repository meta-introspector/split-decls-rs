macro_rules! UnknownLintSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnknownLintSuggestion { # [suggestion (lint_suggestion , code = "{replace}" , applicability = "maybe-incorrect")] WithSpan { # [primary_span] suggestion : Span , replace : Symbol , from_rustc : bool , } , # [help (lint_help)] WithoutSpan { replace : Symbol , from_rustc : bool } , }
    };
}

UnknownLintSuggestion!()