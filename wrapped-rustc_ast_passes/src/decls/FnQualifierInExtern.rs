macro_rules! FnQualifierInExtern {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_extern_fn_qualifiers)] pub (crate) struct FnQualifierInExtern { # [primary_span] # [suggestion (code = "" , applicability = "maybe-incorrect")] pub span : Span , # [label] pub block : Span , pub kw : & 'static str , }
    };
}

FnQualifierInExtern!()