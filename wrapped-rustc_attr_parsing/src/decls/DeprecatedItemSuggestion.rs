macro_rules! DeprecatedItemSuggestion {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_deprecated_item_suggestion)] pub (crate) struct DeprecatedItemSuggestion { # [primary_span] pub span : Span , # [help] pub is_nightly : bool , # [note] pub details : () , }
    };
}

DeprecatedItemSuggestion!()