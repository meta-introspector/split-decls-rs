macro_rules! FunctionNotFoundInTrait {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_function_not_found_in_trait)] pub (crate) struct FunctionNotFoundInTrait { # [primary_span] pub span : Span , }
    };
}

FunctionNotFoundInTrait!()