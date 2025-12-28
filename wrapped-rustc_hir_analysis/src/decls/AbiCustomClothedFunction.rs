macro_rules! AbiCustomClothedFunction {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_abi_custom_clothed_function)] pub (crate) struct AbiCustomClothedFunction { # [primary_span] pub span : Span , # [suggestion (hir_analysis_suggestion , applicability = "maybe-incorrect" , code = "#[unsafe(naked)]\n" , style = "short")] pub naked_span : Span , }
    };
}

AbiCustomClothedFunction!();