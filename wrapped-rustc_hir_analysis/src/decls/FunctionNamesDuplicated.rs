macro_rules! FunctionNamesDuplicated {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_functions_names_duplicated)] # [note] pub (crate) struct FunctionNamesDuplicated { # [primary_span] pub spans : Vec < Span > , }
    };
}

FunctionNamesDuplicated!()