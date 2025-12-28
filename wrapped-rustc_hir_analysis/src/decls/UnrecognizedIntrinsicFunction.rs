macro_rules! UnrecognizedIntrinsicFunction {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_unrecognized_intrinsic_function , code = E0093)] # [help] pub (crate) struct UnrecognizedIntrinsicFunction { # [primary_span] # [label] pub span : Span , pub name : Symbol , }
    };
}

UnrecognizedIntrinsicFunction!()