macro_rules! AssocKindMismatchWrapInBracesSugg {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_analysis_assoc_kind_mismatch_wrap_in_braces_sugg , applicability = "maybe-incorrect")] pub (crate) struct AssocKindMismatchWrapInBracesSugg { # [suggestion_part (code = "{{ ")] pub lo : Span , # [suggestion_part (code = " }}")] pub hi : Span , }
    };
}

AssocKindMismatchWrapInBracesSugg!()