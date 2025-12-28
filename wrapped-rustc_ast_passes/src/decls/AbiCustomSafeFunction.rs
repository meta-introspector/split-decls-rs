macro_rules! AbiCustomSafeFunction {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_abi_custom_safe_function)] pub (crate) struct AbiCustomSafeFunction { # [primary_span] pub span : Span , pub abi : ExternAbi , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "unsafe " , style = "verbose")] pub unsafe_span : Span , }
    };
}

AbiCustomSafeFunction!();