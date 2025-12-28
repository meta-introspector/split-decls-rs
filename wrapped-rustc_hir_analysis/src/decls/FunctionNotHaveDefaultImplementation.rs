macro_rules! FunctionNotHaveDefaultImplementation {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_function_not_have_default_implementation)] pub (crate) struct FunctionNotHaveDefaultImplementation { # [primary_span] pub span : Span , # [note] pub note_span : Span , }
    };
}

FunctionNotHaveDefaultImplementation!()