macro_rules! DuplicatePreciseCapturing {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_precise_capturing_duplicated)] pub (crate) struct DuplicatePreciseCapturing { # [primary_span] pub bound1 : Span , # [label] pub bound2 : Span , }
    };
}

DuplicatePreciseCapturing!()