macro_rules! MainFunctionReturnTypeGeneric {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_main_function_return_type_generic , code = E0131)] pub (crate) struct MainFunctionReturnTypeGeneric { # [primary_span] pub span : Span , }
    };
}

MainFunctionReturnTypeGeneric!();