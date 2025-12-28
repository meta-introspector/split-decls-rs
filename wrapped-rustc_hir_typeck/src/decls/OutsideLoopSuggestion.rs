macro_rules! OutsideLoopSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_outside_loop_suggestion , applicability = "maybe-incorrect")] pub (crate) struct OutsideLoopSuggestion { # [suggestion_part (code = "'block: ")] pub block_span : Span , # [suggestion_part (code = " 'block")] pub break_spans : Vec < Span > , }
    };
}

OutsideLoopSuggestion!()