macro_rules! LossyProvenanceInt2PtrSuggestion {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_suggestion , applicability = "has-placeholders")] pub (crate) struct LossyProvenanceInt2PtrSuggestion { # [suggestion_part (code = "(...).with_addr(")] pub lo : Span , # [suggestion_part (code = ")")] pub hi : Span , }
    };
}

LossyProvenanceInt2PtrSuggestion!()