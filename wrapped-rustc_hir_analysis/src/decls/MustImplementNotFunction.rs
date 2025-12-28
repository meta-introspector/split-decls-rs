macro_rules! deps {
    () => {
        MustImplementNotFunctionSpanNote!();
        MustImplementNotFunctionNote!();
    };
}

macro_rules! MustImplementNotFunction {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_must_implement_not_function)] pub (crate) struct MustImplementNotFunction { # [primary_span] pub span : Span , # [subdiagnostic] pub span_note : MustImplementNotFunctionSpanNote , # [subdiagnostic] pub note : MustImplementNotFunctionNote , }
    };
}

MustImplementNotFunction!()