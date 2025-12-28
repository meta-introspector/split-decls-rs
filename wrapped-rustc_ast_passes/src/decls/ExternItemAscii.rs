macro_rules! ExternItemAscii {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_extern_item_ascii)] # [note] pub (crate) struct ExternItemAscii { # [primary_span] pub span : Span , # [label] pub block : Span , }
    };
}

ExternItemAscii!();