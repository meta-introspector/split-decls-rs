macro_rules! deps {
    () => {
        ExternBlockSuggestion!();
    };
}

macro_rules! FnWithoutBody {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (ast_passes_fn_without_body)] pub (crate) struct FnWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " {{ <body> }}" , applicability = "has-placeholders")] pub replace_span : Span , # [subdiagnostic] pub extern_block_suggestion : Option < ExternBlockSuggestion > , }
    };
}

FnWithoutBody!();