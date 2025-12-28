macro_rules! MustImplementNotFunctionSpanNote {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_analysis_must_implement_not_function_span_note)] pub (crate) struct MustImplementNotFunctionSpanNote { # [primary_span] pub span : Span , }
    };
}

MustImplementNotFunctionSpanNote!();