macro_rules! PassFnItemToVariadicFunction {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_fn_item_to_variadic_function , code = E0617)] # [help] # [note] pub (crate) struct PassFnItemToVariadicFunction { # [primary_span] pub span : Span , # [suggestion (code = " as {replace}" , applicability = "machine-applicable" , style = "verbose")] pub sugg_span : Span , pub replace : String , }
    };
}

PassFnItemToVariadicFunction!();