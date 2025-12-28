macro_rules! VariadicFunctionCompatibleConvention {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_variadic_function_compatible_convention , code = E0045)] pub (crate) struct VariadicFunctionCompatibleConvention < 'a > { # [primary_span] # [label] pub span : Span , pub convention : & 'a str , }
    };
}

VariadicFunctionCompatibleConvention!();