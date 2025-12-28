macro_rules! AbiCustomSafeForeignFunction {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_abi_custom_safe_foreign_function)] pub (crate) struct AbiCustomSafeForeignFunction { # [primary_span] pub span : Span , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "" , style = "verbose")] pub safe_span : Span , }
    };
}

AbiCustomSafeForeignFunction!();