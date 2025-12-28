macro_rules! ExternBlockSuggestion {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum ExternBlockSuggestion { # [multipart_suggestion (ast_passes_extern_block_suggestion , applicability = "maybe-incorrect")] Implicit { # [suggestion_part (code = "extern {{")] start_span : Span , # [suggestion_part (code = " }}")] end_span : Span , } , # [multipart_suggestion (ast_passes_extern_block_suggestion , applicability = "maybe-incorrect")] Explicit { # [suggestion_part (code = "extern \"{abi}\" {{")] start_span : Span , # [suggestion_part (code = " }}")] end_span : Span , abi : Symbol , } , }
    };
}

ExternBlockSuggestion!();