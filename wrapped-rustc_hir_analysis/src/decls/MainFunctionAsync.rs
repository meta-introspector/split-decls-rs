macro_rules! MainFunctionAsync {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_main_function_async , code = E0752)] pub (crate) struct MainFunctionAsync { # [primary_span] pub span : Span , # [label] pub asyncness : Option < Span > , }
    };
}

MainFunctionAsync!();