macro_rules! MainFunctionGenericParameters {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_main_function_generic_parameters , code = E0131)] pub (crate) struct MainFunctionGenericParameters { # [primary_span] pub span : Span , # [label] pub label_span : Option < Span > , }
    };
}

MainFunctionGenericParameters!();